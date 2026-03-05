use crate::ast::{ConstraintExpr, RawBlock, RawItem, RawValue, Span, Spanned, TagRef, TypeRefinement};
use crate::meta::{Annotation, SourceAnnotation};
use crate::parser::meta::{MetaBlock, MetaConstraint, MetaField, MetaItem, MetaTypeDef};
use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct ValidationError {
    pub span: Span,
    pub message: String,
}

impl ValidationError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ValidationError {}

/// Built-in type names
const BUILTIN_TYPES: &[&str] = &["String", "Int", "Float", "Bool", "Type"];

/// Check if a type name is a built-in type
fn is_builtin_type(name: &str) -> bool {
    BUILTIN_TYPES.iter().any(|&t| t.eq_ignore_ascii_case(name))
}

/// Check if a string starts with uppercase (PascalCase)
fn is_pascal_case(s: &str) -> bool {
    s.chars().next().map(|c| c.is_ascii_uppercase()).unwrap_or(false)
}

/// Validate a schema value against a meta type
fn validate_type(
    meta_type: &RawValue,
    schema_value: &Spanned<RawValue>,
    meta_map: &HashMap<String, &MetaBlock>,
    type_defs_map: &HashMap<String, &MetaTypeDef>,
    errors: &mut Vec<ValidationError>,
) {
    match meta_type {
        // Type - expects any PascalCase type reference
        RawValue::Type(t) if t == "Type" => {
            match &schema_value.0 {
                RawValue::Ref(parts) => {
                    if let Some(first) = parts.first() {
                        if !is_pascal_case(first) {
                            errors.push(ValidationError::new(
                                schema_value.span().clone(),
                                format!("expected Type (PascalCase), found '{}'", first),
                            ));
                        }
                    }
                }
                _ => {
                    errors.push(ValidationError::new(
                        schema_value.span().clone(),
                        "expected Type reference".to_string(),
                    ));
                }
            }
        }
        // Built-in types (String, Int, Float, Bool) - expects type reference
        RawValue::Type(t) if is_builtin_type(t) => {
            match &schema_value.0 {
                RawValue::Ref(parts) if parts.len() == 1 => {
                    if !parts[0].eq_ignore_ascii_case(t) {
                        errors.push(ValidationError::new(
                            schema_value.span().clone(),
                            format!("expected {} type, found '{}'", t, parts[0]),
                        ));
                    }
                }
                _ => {
                    errors.push(ValidationError::new(
                        schema_value.span().clone(),
                        format!("expected {} type reference", t),
                    ));
                }
            }
        }
        // User-defined type - look up in blocks or type defs
        RawValue::Type(t) => {
            // If schema value is a Ref, it's a reference to a block - accept it
            // (The block's type should match, but we trust schema parser to enforce that)
            if matches!(&schema_value.0, RawValue::Ref(_) | RawValue::TypeRefinement(_)) {
                return;
            }

            let key = t.to_ascii_lowercase();
            if let Some(meta_def) = meta_map.get(&key) {
                validate_schema_against_meta_block(meta_def, schema_value, meta_map, type_defs_map, errors);
            } else if let Some(type_def) = type_defs_map.get(&key) {
                // Type def: validate against its value
                validate_type(&type_def.value.0, schema_value, meta_map, type_defs_map, errors);
            }
            // If not found, assume it's valid (could be external type)
        }
        // Concrete<T> - expects concrete value
        RawValue::Concrete(inner) => {
            if let RawValue::Type(t) = &inner.0 {
                match t.as_str() {
                    "String" => {
                        if !matches!(&schema_value.0, RawValue::String(_)) {
                            errors.push(ValidationError::new(
                                schema_value.span().clone(),
                                "expected concrete string value".to_string(),
                            ));
                        }
                    }
                    "Int" => {
                        if !matches!(&schema_value.0, RawValue::Int(_)) {
                            errors.push(ValidationError::new(
                                schema_value.span().clone(),
                                "expected concrete int value".to_string(),
                            ));
                        }
                    }
                    "Float" => {
                        if !matches!(&schema_value.0, RawValue::Float(_)) {
                            errors.push(ValidationError::new(
                                schema_value.span().clone(),
                                "expected concrete float value".to_string(),
                            ));
                        }
                    }
                    "Bool" => {
                        if !matches!(&schema_value.0, RawValue::Bool(_)) {
                            errors.push(ValidationError::new(
                                schema_value.span().clone(),
                                "expected concrete bool value".to_string(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        // Union - schema value must match one of the variants
        RawValue::Union(variants) => {
            let mut variant_errors: Vec<Vec<ValidationError>> = Vec::new();
            for variant in variants {
                let mut errs = Vec::new();
                validate_type(&variant.0, schema_value, meta_map, type_defs_map, &mut errs);
                if errs.is_empty() {
                    return; // Found matching variant
                }
                variant_errors.push(errs);
            }
            // None matched
            errors.push(ValidationError::new(
                schema_value.span().clone(),
                "value doesn't match any variant of union type".to_string(),
            ));
        }
        // Intersection - schema value must match all parts
        RawValue::Intersection(parts) => {
            for part in parts {
                validate_type(&part.0, schema_value, meta_map, type_defs_map, errors);
            }
        }
        // WildcardObject {* _ Type} or {1 _ Type}
        RawValue::WildcardObject { value, .. } => {
            if let RawValue::Object(fields) = &schema_value.0 {
                for (_, field_value) in fields {
                    validate_type(&value.0, field_value, meta_map, type_defs_map, errors);
                }
            } else {
                errors.push(ValidationError::new(
                    schema_value.span().clone(),
                    "expected object".to_string(),
                ));
            }
        }
        // List []T
        RawValue::List(items) if items.len() == 1 => {
            if let RawValue::List(schema_items) = &schema_value.0 {
                for item in schema_items {
                    validate_type(&items[0].0, item, meta_map, type_defs_map, errors);
                }
            } else {
                errors.push(ValidationError::new(
                    schema_value.span().clone(),
                    "expected list".to_string(),
                ));
            }
        }
        _ => {}
    }
}

/// Validate schema value against a meta block definition
fn validate_schema_against_meta_block(
    meta_def: &MetaBlock,
    schema_value: &Spanned<RawValue>,
    meta_map: &HashMap<String, &MetaBlock>,
    type_defs_map: &HashMap<String, &MetaTypeDef>,
    errors: &mut Vec<ValidationError>,
) {
    match &schema_value.0 {
        RawValue::Object(fields) => {
            // Find wildcard field if any
            let wildcard_field = meta_def.body.iter().find_map(|item| {
                if let MetaItem::Field(f) = &item.0 {
                    if f.name.value() == "_" {
                        return Some(f);
                    }
                }
                None
            });

            for (name, value) in fields {
                // Find matching meta field
                let meta_field = meta_def.body.iter().find_map(|item| {
                    if let MetaItem::Field(f) = &item.0 {
                        if f.name.value() == name.value() {
                            return Some(f);
                        }
                    }
                    None
                });

                if let Some(mf) = meta_field.or(wildcard_field) {
                    validate_type(&mf.value.0, value, meta_map, type_defs_map, errors);
                }
            }
        }
        RawValue::Ref(_) => {
            // Reference to another block - accept refs
        }
        _ => {}
    }
}

/// Validates @source annotations on schema blocks against meta.
/// For fields with @source [sources], validates that emitted event fields
/// exist in the source blocks with matching types (unless marked generated).
pub fn validate_sources(meta: &[MetaBlock], schema: &[RawBlock]) -> Vec<ValidationError> {
    validate_sources_with_type_defs(meta, &[], schema)
}

/// Validates schema against meta blocks and type definitions.
pub fn validate_sources_with_type_defs(
    meta: &[MetaBlock],
    type_defs: &[MetaTypeDef],
    schema: &[RawBlock],
) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Build lookup for meta blocks (lowercase keys for case-insensitive matching)
    let meta_map: HashMap<String, &MetaBlock> = meta
        .iter()
        .map(|b| (b.kind.value().to_ascii_lowercase(), b))
        .collect();

    // Build lookup for type defs
    let type_defs_map: HashMap<String, &MetaTypeDef> = type_defs
        .iter()
        .map(|t| (t.name.value().to_ascii_lowercase(), t))
        .collect();

    // Build lookup for schema blocks (for event definitions)
    let schema_map: HashMap<&str, &RawBlock> = schema
        .iter()
        .filter_map(|b| b.name.as_ref().map(|n| (n.value().as_str(), b)))
        .collect();

    // Build event -> tags association map (needed for both tag validation and constraints)
    let event_tags = build_event_tags_map(schema);

    // Validate tag associations
    validate_tag_associations_with_map(schema, &schema_map, &event_tags, &mut errors);

    for block in schema {
        let key = block.kind.value().to_ascii_lowercase();
        if let Some(meta_def) = meta_map.get(&key) {
            validate_block(meta_def, block, &meta_map, &type_defs_map, &schema_map, &mut errors);
            validate_constraints(meta_def, block, &schema_map, &event_tags, &mut errors);
            // Also validate constraints on nested values (like QueryItem objects)
            validate_nested_constraints(meta_def, block, &meta_map, &type_defs_map, &schema_map, &event_tags, &mut errors);
        } else if let Some(type_def) = type_defs_map.get(&key) {
            // Block kind matches a type def - validate block body against type def value
            validate_block_against_type_def(type_def, block, &meta_map, &type_defs_map, &mut errors);
        }
    }

    errors
}

/// Validate a block against a type definition
fn validate_block_against_type_def(
    type_def: &MetaTypeDef,
    block: &RawBlock,
    meta_map: &HashMap<String, &MetaBlock>,
    type_defs_map: &HashMap<String, &MetaTypeDef>,
    errors: &mut Vec<ValidationError>,
) {
    // Convert block body to a schema value for validation
    let schema_value = match &block.body {
        crate::ast::BlockBody::Items(items) => {
            // Convert items to Object
            let fields: Vec<(Spanned<String>, Spanned<RawValue>)> = items
                .iter()
                .filter_map(|item| {
                    if let RawItem::Field(f) = &item.0 {
                        Some((f.name.clone(), f.value.clone()))
                    } else {
                        None
                    }
                })
                .collect();
            Spanned::new(RawValue::Object(fields), block.kind.span().clone())
        }
        crate::ast::BlockBody::Value(v) => v.clone(),
    };

    validate_type(&type_def.value.0, &schema_value, meta_map, type_defs_map, errors);
}

/// Build event -> tags association map
fn build_event_tags_map<'a>(schema: &'a [RawBlock]) -> HashMap<&'a str, Vec<&'a str>> {
    // Collect declared tags
    let tags: HashSet<&str> = schema
        .iter()
        .filter(|b| b.kind.value().eq_ignore_ascii_case("tag"))
        .filter_map(|b| b.name.as_ref().map(|n| n.value().as_str()))
        .collect();

    let mut event_tags: HashMap<&str, Vec<&str>> = HashMap::new();

    for block in schema {
        if let Some(name) = block.name.as_ref() {
            for assoc in &block.associations {
                match assoc.value() {
                    TagRef::Ident(tag_name) => {
                        if tags.contains(tag_name.as_str()) {
                            event_tags
                                .entry(name.value().as_str())
                                .or_default()
                                .push(tag_name.as_str());
                        }
                    }
                    TagRef::String(s) => {
                        event_tags
                            .entry(name.value().as_str())
                            .or_default()
                            .push(s.as_str());
                    }
                }
            }
        }
    }

    event_tags
}

/// Validates tag associations:
/// 1. Tags referenced in associations must exist
/// 2. Query tags must be associated with ALL queried event types
fn validate_tag_associations_with_map(
    schema: &[RawBlock],
    schema_map: &HashMap<&str, &RawBlock>,
    event_tags: &HashMap<&str, Vec<&str>>,
    errors: &mut Vec<ValidationError>,
) {
    // Collect declared tags (blocks with kind "tag")
    let tags: HashSet<&str> = schema
        .iter()
        .filter(|b| b.kind.value().eq_ignore_ascii_case("tag"))
        .filter_map(|b| b.name.as_ref().map(|n| n.value().as_str()))
        .collect();

    // Validate associations exist
    for block in schema {
        for assoc in &block.associations {
            if let TagRef::Ident(tag_name) = assoc.value() {
                if !tags.contains(tag_name.as_str()) {
                    errors.push(ValidationError::new(
                        assoc.span().clone(),
                        format!("tag '{}' not declared", tag_name),
                    ));
                }
            }
        }
    }

    // Validate queries: tags must be associated with ALL eventTypes
    for block in schema {
        // Check items body
        for item in block.body.items() {
            if let RawItem::Field(field) = &item.0 {
                if field.name.value() == "query" {
                    validate_query_tags(&field.value, event_tags, schema_map, errors);
                }
            }
            // Check nested blocks with value body (e.g., query { [...] })
            if let RawItem::Block(nested) = &item.0 {
                if nested.kind.value() == "query" {
                    if let Some(v) = nested.body.value() {
                        validate_query_tags(v, event_tags, schema_map, errors);
                    }
                }
            }
        }
    }
}

/// Validate that tags in query are associated with all eventTypes
fn validate_query_tags(
    query_value: &Spanned<RawValue>,
    event_tags: &HashMap<&str, Vec<&str>>,
    schema_map: &HashMap<&str, &RawBlock>,
    errors: &mut Vec<ValidationError>,
) {
    if let RawValue::List(items) = &query_value.0 {
        for item in items {
            if let RawValue::Object(fields) = &item.0 {
                let mut event_types: Vec<&str> = vec![];
                let mut query_tags: Vec<(&Spanned<RawValue>, &str)> = vec![];

                for (name, value) in fields {
                    if name.value() == "eventTypes" {
                        if let RawValue::List(events) = &value.0 {
                            for ev in events {
                                if let RawValue::Ref(parts) = &ev.0 {
                                    if let Some(name) = parts.first() {
                                        event_types.push(name.as_str());
                                    }
                                }
                            }
                        }
                    } else if name.value() == "tags" {
                        if let RawValue::List(tag_list) = &value.0 {
                            for tag in tag_list {
                                match &tag.0 {
                                    RawValue::Ref(parts) => {
                                        if let Some(tag_name) = parts.first() {
                                            query_tags.push((tag, tag_name.as_str()));
                                        }
                                    }
                                    RawValue::String(s) => {
                                        query_tags.push((tag, s.as_str()));
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }

                // Validate each query tag is associated with ALL eventTypes
                for (tag_spanned, tag_name) in &query_tags {
                    for event_type in &event_types {
                        // Check if event exists
                        if !schema_map.contains_key(event_type) {
                            continue; // Other validation catches missing events
                        }

                        let event_assocs = event_tags.get(event_type).map(|v| v.as_slice()).unwrap_or(&[]);
                        if !event_assocs.contains(tag_name) {
                            errors.push(ValidationError::new(
                                tag_spanned.span().clone(),
                                format!(
                                    "tag '{}' not associated with event '{}'",
                                    tag_name, event_type
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn validate_block(
    meta_def: &MetaBlock,
    block: &RawBlock,
    meta_map: &HashMap<String, &MetaBlock>,
    type_defs_map: &HashMap<String, &MetaTypeDef>,
    schema_map: &HashMap<&str, &RawBlock>,
    errors: &mut Vec<ValidationError>,
) {
    // Collect source fields from this block
    let source_fields = collect_source_fields(meta_def, block);

    // Check if meta has a wildcard field (allows any field name)
    let has_wildcard = meta_def.body.iter().any(|item| {
        matches!(&item.0, MetaItem::Field(f) if f.name.value() == "_")
    });

    for item in block.body.items() {
        match &item.0 {
            RawItem::Field(field) => {
                // Validate field exists in meta (if no wildcard)
                let meta_field = find_meta_field(meta_def, &field.name.0);
                if meta_field.is_none() && !has_wildcard {
                    errors.push(ValidationError::new(
                        field.name.span().clone(),
                        format!(
                            "field '{}' not defined in meta for '{}'",
                            field.name.0,
                            meta_def.kind.value()
                        ),
                    ));
                    continue;
                }

                // Get the meta field (either named or wildcard)
                let mf = meta_field.or_else(|| {
                    if has_wildcard {
                        meta_def.body.iter().find_map(|item| {
                            if let MetaItem::Field(f) = &item.0 {
                                if f.name.value() == "_" {
                                    return Some(f);
                                }
                            }
                            None
                        })
                    } else {
                        None
                    }
                });

                if let Some(mf) = mf {
                    // Validate type
                    validate_type(&mf.value.0, &field.value, meta_map, type_defs_map, errors);

                    // Check @source annotation validation
                    if let Some(src_ann) = get_source_annotation(&mf.annotations) {
                        let for_paths = src_ann.for_paths.as_ref();
                        match &field.value.0 {
                            RawValue::List(items) => {
                                for item in items {
                                    validate_source_target(item, &source_fields, schema_map, for_paths, errors);
                                }
                            }
                            _ => validate_source_target(&field.value, &source_fields, schema_map, for_paths, errors),
                        }
                    }

                    // Validate nested type references (e.g., []queryItem)
                    if let Some(ref_type) = get_referenced_meta_type(&mf.value.0) {
                        if let Some(ref_meta) = meta_map.get(&ref_type.to_ascii_lowercase()) {
                            validate_value_against_meta(ref_meta, &field.value, meta_map, type_defs_map, errors);
                        }
                    }
                }
            }
            RawItem::Block(nested) => {
                // Validate nested block exists in meta (as block OR field with object-like value)
                let meta_nested = find_meta_block(meta_def, &nested.kind.0);
                let meta_field = find_meta_field(meta_def, &nested.kind.0);
                if meta_nested.is_none() && meta_field.is_none() && !has_wildcard {
                    errors.push(ValidationError::new(
                        nested.kind.span().clone(),
                        format!(
                            "block '{}' not defined in meta for '{}'",
                            nested.kind.0,
                            meta_def.kind.value()
                        ),
                    ));
                }

                // Handle blocks with value body (e.g., emits [...])
                if let Some(mf) = meta_field {
                    if let Some(body_value) = nested.body.value() {
                        if let Some(src_ann) = get_source_annotation(&mf.annotations) {
                            let for_paths = src_ann.for_paths.as_ref();
                            if let RawValue::List(items) = &body_value.0 {
                                for item in items {
                                    validate_source_target(item, &source_fields, schema_map, for_paths, errors);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Extract referenced meta type from meta value (e.g., []queryItem -> "queryItem")
fn get_referenced_meta_type(value: &RawValue) -> Option<&str> {
    match value {
        RawValue::List(items) if items.len() == 1 => {
            if let RawValue::Type(t) = &items[0].0 {
                Some(t.as_str())
            } else {
                None
            }
        }
        RawValue::Type(t) => Some(t.as_str()),
        _ => None,
    }
}

/// Validate a schema value against a meta block definition
fn validate_value_against_meta(
    meta_def: &MetaBlock,
    value: &Spanned<RawValue>,
    meta_map: &HashMap<String, &MetaBlock>,
    type_defs_map: &HashMap<String, &MetaTypeDef>,
    errors: &mut Vec<ValidationError>,
) {
    match &value.0 {
        RawValue::List(items) => {
            for item in items {
                validate_value_against_meta(meta_def, item, meta_map, type_defs_map, errors);
            }
        }
        RawValue::Object(fields) => {
            let has_wildcard = meta_def.body.iter().any(|item| {
                matches!(&item.0, MetaItem::Field(f) if f.name.value() == "_")
            });

            for (name, field_value) in fields {
                let meta_field = find_meta_field(meta_def, name.value());
                if meta_field.is_none() && !has_wildcard {
                    errors.push(ValidationError::new(
                        name.span().clone(),
                        format!(
                            "field '{}' not defined in meta for '{}'",
                            name.value(),
                            meta_def.kind.value()
                        ),
                    ));
                    continue;
                }

                // Recursively validate nested references
                if let Some(mf) = meta_field {
                    if let Some(ref_type) = get_referenced_meta_type(&mf.value.0) {
                        if let Some(ref_meta) = meta_map.get(&ref_type.to_ascii_lowercase()) {
                            validate_value_against_meta(ref_meta, field_value, meta_map, type_defs_map, errors);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn find_meta_field<'a>(meta_def: &'a MetaBlock, name: &str) -> Option<&'a MetaField> {
    for item in &meta_def.body {
        if let MetaItem::Field(f) = &item.0 {
            // Match by name or wildcard
            if f.name.value() == name || f.name.value() == "_" {
                return Some(f);
            }
        }
    }
    None
}

fn find_meta_block<'a>(meta_def: &'a MetaBlock, kind: &str) -> Option<&'a MetaBlock> {
    for item in &meta_def.body {
        if let MetaItem::Block(b) = &item.0 {
            if b.kind.value() == kind {
                return Some(b);
            }
        }
    }
    None
}

fn get_source_annotation(annotations: &[Annotation]) -> Option<&SourceAnnotation> {
    annotations.iter().find_map(|a| match a {
        Annotation::Source(src) => Some(src),
        _ => None,
    })
}

#[derive(Debug, Clone)]
struct FieldInfo {
    type_name: String,
    optional: bool,
    #[allow(dead_code)]
    generated: bool,
}

/// Collect all fields available from source blocks
fn collect_source_fields(meta_def: &MetaBlock, block: &RawBlock) -> HashMap<String, FieldInfo> {
    let mut fields = HashMap::new();

    // Find which nested blocks are sources from meta
    let source_names: Vec<&str> = meta_def
        .body
        .iter()
        .filter_map(|item| {
            if let MetaItem::Field(f) = &item.0 {
                get_source_annotation(&f.annotations)
                    .map(|src| src.sources.iter().map(|s| s.as_str()))
            } else {
                None
            }
        })
        .flatten()
        .collect();

    // Find source data in schema - can be either:
    // 1. Nested Block with matching kind name
    // 2. Field with Object value and matching field name (parser ambiguity)
    for item in block.body.items() {
        match &item.0 {
            RawItem::Block(nested_block) => {
                if source_names.contains(&nested_block.kind.value().as_str()) {
                    collect_fields_from_block(nested_block, &mut fields);
                }
            }
            RawItem::Field(field) => {
                // Handle case where parser parsed "fields { ... }" as a field with Object value
                if source_names.contains(&field.name.value().as_str()) {
                    if let RawValue::Object(obj_fields) = &field.value.0 {
                        for (name, value) in obj_fields {
                            if let Some(type_name) = extract_type_name(value.value()) {
                                fields.insert(
                                    name.value().clone(),
                                    FieldInfo {
                                        type_name,
                                        optional: false,
                                        generated: false,
                                    },
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fields
}

fn collect_fields_from_block(block: &RawBlock, fields: &mut HashMap<String, FieldInfo>) {
    for item in block.body.items() {
        if let RawItem::Field(field) = &item.0 {
            if let Some(type_name) = extract_type_name(&field.value.0) {
                fields.insert(
                    field.name.0.clone(),
                    FieldInfo {
                        type_name,
                        optional: field.optional,
                        generated: field.generated,
                    },
                );
            }
        }
    }
}

fn extract_type_name(value: &RawValue) -> Option<String> {
    match value {
        RawValue::Ref(parts) => Some(parts.join(".")),
        RawValue::Type(t) => Some(t.clone()),
        RawValue::Ident(i) => Some(i.clone()),
        _ => None,
    }
}

fn validate_source_target(
    target: &crate::ast::Spanned<RawValue>,
    source_fields: &HashMap<String, FieldInfo>,
    schema_map: &HashMap<&str, &RawBlock>,
    for_paths: Option<&Vec<String>>,
    errors: &mut Vec<ValidationError>,
) {
    match &target.0 {
        // Simple ref: emits [UserRegistered]
        RawValue::Ref(parts) => {
            let name = parts.join(".");
            if let Some(block) = schema_map.get(name.as_str()) {
                validate_target_fields_covered(
                    block,
                    None,
                    source_fields,
                    target.span(),
                    errors,
                );
            }
        }
        // TypeRefinement: emits [UserRegistered{timestamp Int*}]
        RawValue::TypeRefinement(refinement) => {
            if let Some(block) = schema_map.get(refinement.base.as_str()) {
                validate_target_fields_covered(
                    block,
                    Some(refinement),
                    source_fields,
                    target.span(),
                    errors,
                );
            }
        }
        // Object: recurse into fields (filtered by for_paths if specified)
        RawValue::Object(fields) => {
            for (name, value) in fields {
                let should_validate = match for_paths {
                    Some(paths) => paths.iter().any(|p| p == name.value()),
                    None => true, // validate all fields if no for_paths
                };
                if should_validate {
                    // Recurse without for_paths - we've filtered or are validating all
                    validate_source_target(value, source_fields, schema_map, None, errors);
                }
            }
        }
        // List: always recurse into items
        RawValue::List(items) => {
            for item in items {
                validate_source_target(item, source_fields, schema_map, for_paths, errors);
            }
        }
        // Union: validate each variant
        RawValue::Union(variants) => {
            for v in variants {
                validate_source_target(v, source_fields, schema_map, for_paths, errors);
            }
        }
        // Intersection: validate each part
        RawValue::Intersection(parts) => {
            for p in parts {
                validate_source_target(p, source_fields, schema_map, for_paths, errors);
            }
        }
        _ => {}
    }
}

struct OverrideInfo {
    is_generated: bool,
    type_name: Option<String>,
}

fn extract_override_info(value: &RawValue) -> OverrideInfo {
    match value {
        RawValue::Object(fields) => {
            let is_generated = fields.iter().any(|(k, v)| {
                k.value() == "__generated" && matches!(v.value(), RawValue::Bool(true))
            });
            let type_name = fields.iter().find_map(|(k, v)| {
                if k.value() == "__type" {
                    extract_type_name(v.value())
                } else {
                    None
                }
            });
            OverrideInfo {
                is_generated,
                type_name,
            }
        }
        _ => OverrideInfo {
            is_generated: false,
            type_name: extract_type_name(value),
        },
    }
}

fn validate_target_fields_covered(
    event_block: &RawBlock,
    refinement: Option<&TypeRefinement>,
    source_fields: &HashMap<String, FieldInfo>,
    span: &Span,
    errors: &mut Vec<ValidationError>,
) {
    // Get overrides from refinement
    let overrides: HashMap<&str, OverrideInfo> = refinement
        .map(|r| {
            r.overrides
                .iter()
                .map(|(name, value)| (name.value().as_str(), extract_override_info(value.value())))
                .collect()
        })
        .unwrap_or_default();

    // Get event name for error messages
    let event_name = event_block
        .name
        .as_ref()
        .map(|n| n.value().as_str())
        .unwrap_or("unknown");

    // Check each field in event definition
    for item in event_block.body.items() {
        if let RawItem::Field(field) = &item.0 {
            let field_name = &field.name.0;

            // Skip if field is marked generated in event def
            if field.generated {
                continue;
            }

            // Check if override exists
            if let Some(override_info) = overrides.get(field_name.as_str()) {
                // Validate override type matches event field type
                if let Some(expected_type) = extract_type_name(&field.value.0) {
                    if let Some(ref override_type) = override_info.type_name {
                        if override_type != &expected_type {
                            errors.push(ValidationError::new(
                                span.clone(),
                                format!(
                                    "field '{}' override type mismatch: override has '{}', event expects '{}'",
                                    field_name, override_type, expected_type
                                ),
                            ));
                        }
                    }
                }
                // Generated overrides don't need source validation
                if override_info.is_generated {
                    continue;
                }
            }

            // Optional target field: source not existing is OK
            if field.optional && !source_fields.contains_key(field_name) {
                continue;
            }

            // Field must exist in source for required fields
            if !source_fields.contains_key(field_name) {
                errors.push(ValidationError::new(
                    span.clone(),
                    format!(
                        "field '{}' in event '{}' not found in source and not marked as generated",
                        field_name, event_name
                    ),
                ));
                continue;
            }

            let source_info = &source_fields[field_name];

            // Required target cannot have optional source
            if !field.optional && source_info.optional {
                errors.push(ValidationError::new(
                    span.clone(),
                    format!(
                        "required field '{}' in event '{}' cannot be sourced from optional field",
                        field_name, event_name
                    ),
                ));
                continue;
            }

            // Type check
            if let Some(expected_type) = extract_type_name(&field.value.0) {
                if source_info.type_name != expected_type {
                    errors.push(ValidationError::new(
                        span.clone(),
                        format!(
                            "field '{}' type mismatch: source has '{}', event expects '{}'",
                            field_name, source_info.type_name, expected_type
                        ),
                    ));
                }
            }
        }
    }
}

/// Context for evaluating constraint expressions
struct ConstraintContext<'a> {
    /// Current variable bindings: var_name -> (block_name, associations)
    bindings: HashMap<String, (&'a str, &'a HashSet<&'a str>)>,
}

impl<'a> ConstraintContext<'a> {
    fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    fn with_binding(
        &self,
        var: &str,
        block_name: &'a str,
        assocs: &'a HashSet<&'a str>,
    ) -> Self {
        let mut new_ctx = Self {
            bindings: self.bindings.clone(),
        };
        new_ctx.bindings.insert(var.to_string(), (block_name, assocs));
        new_ctx
    }
}

/// Data needed for constraint evaluation
struct ConstraintData<'a> {
    /// field_name -> list of (block_name, associations)
    field_values: HashMap<&'a str, Vec<(&'a str, HashSet<&'a str>)>>,
}

/// Evaluate a constraint expression
fn evaluate_constraint(
    expr: &ConstraintExpr,
    ctx: &ConstraintContext<'_>,
    data: &ConstraintData<'_>,
) -> bool {
    match expr {
        ConstraintExpr::ForAll { field, var, body } => {
            if let Some(values) = data.field_values.get(field.as_str()) {
                values.iter().all(|(name, assocs)| {
                    let new_ctx = ctx.with_binding(var, name, assocs);
                    evaluate_constraint(body, &new_ctx, data)
                })
            } else {
                true // Empty field satisfies forall
            }
        }
        ConstraintExpr::Exists { field, var, body } => {
            if let Some(values) = data.field_values.get(field.as_str()) {
                values.iter().any(|(name, assocs)| {
                    let new_ctx = ctx.with_binding(var, name, assocs);
                    evaluate_constraint(body, &new_ctx, data)
                })
            } else {
                false // Empty field doesn't satisfy exists
            }
        }
        ConstraintExpr::Assoc { subject, target } => {
            // Check if subject is associated with target
            if let (Some((_, subj_assocs)), Some((target_name, _))) =
                (ctx.bindings.get(subject), ctx.bindings.get(target))
            {
                subj_assocs.contains(target_name)
            } else {
                false
            }
        }
        ConstraintExpr::And(left, right) => {
            evaluate_constraint(left, ctx, data) && evaluate_constraint(right, ctx, data)
        }
        ConstraintExpr::Or(left, right) => {
            evaluate_constraint(left, ctx, data) || evaluate_constraint(right, ctx, data)
        }
        ConstraintExpr::Not(inner) => !evaluate_constraint(inner, ctx, data),
    }
}

/// Validate constraints on a block
fn validate_constraints(
    meta_def: &MetaBlock,
    block: &RawBlock,
    schema_map: &HashMap<&str, &RawBlock>,
    event_tags: &HashMap<&str, Vec<&str>>,
    errors: &mut Vec<ValidationError>,
) {
    // Find @constraint items in meta block body
    for item in &meta_def.body {
        if let MetaItem::Constraint(MetaConstraint(expr)) = &item.0 {
            // Build constraint data from schema block values
            let data = build_constraint_data(block, schema_map, event_tags);
            let ctx = ConstraintContext::new();

            if !evaluate_constraint(expr, &ctx, &data) {
                errors.push(ValidationError::new(
                    block.kind.span().clone(),
                    format!(
                        "constraint violation in '{}': constraint not satisfied",
                        block.name.as_ref().map(|n| n.value().as_str()).unwrap_or("unknown")
                    ),
                ));
            }
        }
    }
}

/// Validate constraints on nested field values (e.g., QueryItem objects in query field)
fn validate_nested_constraints(
    meta_def: &MetaBlock,
    block: &RawBlock,
    meta_map: &HashMap<String, &MetaBlock>,
    _type_defs_map: &HashMap<String, &MetaTypeDef>,
    schema_map: &HashMap<&str, &RawBlock>,
    event_tags: &HashMap<&str, Vec<&str>>,
    errors: &mut Vec<ValidationError>,
) {
    // For each field in the schema block
    for item in block.body.items() {
        if let RawItem::Field(field) = &item.0 {
            // Find the meta field definition
            if let Some(meta_field) = find_meta_field(meta_def, field.name.value()) {
                // Get the referenced type (e.g., []QueryItem -> QueryItem)
                if let Some(ref_type) = get_referenced_meta_type(&meta_field.value.0) {
                    // Get the meta definition for that type
                    if let Some(ref_meta) = meta_map.get(&ref_type.to_ascii_lowercase()) {
                        // Check if this meta has constraints
                        let constraints: Vec<_> = ref_meta
                            .body
                            .iter()
                            .filter_map(|item| {
                                if let MetaItem::Constraint(MetaConstraint(expr)) = &item.0 {
                                    Some(expr)
                                } else {
                                    None
                                }
                            })
                            .collect();

                        if !constraints.is_empty() {
                            // Validate constraints on each nested value
                            validate_value_constraints(
                                &field.value,
                                &constraints,
                                schema_map,
                                event_tags,
                                errors,
                            );
                        }
                    }
                }
            }
        }
    }
}

/// Validate constraints on a value (recursively handles lists)
fn validate_value_constraints(
    value: &Spanned<RawValue>,
    constraints: &[&ConstraintExpr],
    schema_map: &HashMap<&str, &RawBlock>,
    event_tags: &HashMap<&str, Vec<&str>>,
    errors: &mut Vec<ValidationError>,
) {
    match &value.0 {
        RawValue::List(items) => {
            for item in items {
                validate_value_constraints(item, constraints, schema_map, event_tags, errors);
            }
        }
        RawValue::Object(fields) => {
            // Build constraint data from this object's fields
            let data = build_constraint_data_from_object(fields, schema_map, event_tags);
            let ctx = ConstraintContext::new();

            for expr in constraints {
                if !evaluate_constraint(expr, &ctx, &data) {
                    errors.push(ValidationError::new(
                        value.span().clone(),
                        "constraint violation: constraint not satisfied".to_string(),
                    ));
                }
            }
        }
        _ => {}
    }
}

/// Build constraint data from an object's fields
fn build_constraint_data_from_object<'a>(
    fields: &'a [(Spanned<String>, Spanned<RawValue>)],
    schema_map: &HashMap<&'a str, &'a RawBlock>,
    event_tags: &HashMap<&'a str, Vec<&'a str>>,
) -> ConstraintData<'a> {
    let mut field_values: HashMap<&str, Vec<(&str, HashSet<&str>)>> = HashMap::new();

    for (name, value) in fields {
        let values = extract_field_refs(value, schema_map, event_tags);
        if !values.is_empty() {
            field_values.insert(name.value().as_str(), values);
        }
    }

    ConstraintData { field_values }
}

/// Build constraint data from schema block values
fn build_constraint_data<'a>(
    block: &'a RawBlock,
    schema_map: &HashMap<&'a str, &'a RawBlock>,
    event_tags: &HashMap<&'a str, Vec<&'a str>>,
) -> ConstraintData<'a> {
    let mut field_values: HashMap<&str, Vec<(&str, HashSet<&str>)>> = HashMap::new();

    for item in block.body.items() {
        if let RawItem::Field(field) = &item.0 {
            let field_name = field.name.value().as_str();
            let values = extract_field_refs(&field.value, schema_map, event_tags);
            if !values.is_empty() {
                field_values.insert(field_name, values);
            }
        }
    }

    ConstraintData { field_values }
}

/// Extract block references from a field value with their associations
fn extract_field_refs<'a>(
    value: &'a Spanned<RawValue>,
    schema_map: &HashMap<&'a str, &'a RawBlock>,
    event_tags: &HashMap<&'a str, Vec<&'a str>>,
) -> Vec<(&'a str, HashSet<&'a str>)> {
    let mut result = Vec::new();

    match &value.0 {
        RawValue::List(items) => {
            for item in items {
                result.extend(extract_field_refs(item, schema_map, event_tags));
            }
        }
        RawValue::Ref(parts) => {
            if let Some(name) = parts.first() {
                let assocs: HashSet<&str> = event_tags
                    .get(name.as_str())
                    .map(|v| v.iter().copied().collect())
                    .unwrap_or_default();
                result.push((name.as_str(), assocs));
            }
        }
        RawValue::Object(fields) => {
            // Handle inline objects like query items: {eventTypes [...], tags [...]}
            for (_, fv) in fields {
                result.extend(extract_field_refs(fv, schema_map, event_tags));
            }
        }
        _ => {}
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::meta::{extract_blocks, extract_type_defs, parse_meta};
    use crate::parser::schema::parse_schema;

    fn get_meta() -> Vec<MetaBlock> {
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
}

command {
    fields {* _ Type}

    @source [fields]
    emits []event

    query []queryItem
}
"#;
        extract_blocks(&parse_meta(input).unwrap())
    }

    #[test]
    fn test_valid_emits_with_generated() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
    timestamp Int
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered{timestamp Int*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_invalid_emits_missing_field() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
    timestamp Int
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(!errors.is_empty(), "expected validation error");
        assert!(errors[0].message.contains("timestamp"));
    }

    #[test]
    fn test_optional_field_without_source_ok() {
        let meta = get_meta();
        // Optional field WITHOUT source should PASS (it's optional)
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
    timestamp? Int
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(
            errors.is_empty(),
            "optional field without source should pass"
        );
    }

    #[test]
    fn test_optional_field_with_source_ok() {
        let meta = get_meta();
        // Optional field WITH matching source field should PASS
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
    timestamp? Int
}

registerUser = Command{
    fields {
        id String
        name String
        timestamp Int
    }
    emits [userRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_optional_field_with_generated_ok() {
        let meta = get_meta();
        // Optional field marked as generated should PASS
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
    timestamp? Int
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered{timestamp Int*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_type_mismatch() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
}

registerUser = Command{
    fields {
        id Int
        name String
    }
    emits [userRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(!errors.is_empty(), "expected type mismatch error");
        assert!(errors[0].message.contains("type mismatch"));
    }

    #[test]
    fn test_required_field_from_optional_source_fails() {
        let meta = get_meta();
        // Required target field sourced from optional source field should FAIL
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
}

registerUser = Command{
    fields {
        id String
        name? String
    }
    emits [userRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(
            !errors.is_empty(),
            "expected error for required field from optional source"
        );
        assert!(
            errors[0].message.contains("required field") && errors[0].message.contains("optional")
        );
    }

    #[test]
    fn test_field_not_in_meta() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userRegistered = Event{
    id String
    name String
}

registerUser = Command{
    fields {
        id String
    }
    unknownField String
    emits [userRegistered{name String*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(!errors.is_empty(), "expected error for unknown field");
        assert!(errors[0].message.contains("unknownField"));
        assert!(errors[0].message.contains("not defined in meta"));
    }

    #[test]
    fn test_tag_not_declared() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userIdTag = Tag{userId String}

userRegistered = Event<userIdTag, undeclaredTag>{
    id String
    name String
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(!errors.is_empty(), "expected error for undeclared tag");
        assert!(errors[0].message.contains("undeclaredTag"));
        assert!(errors[0].message.contains("not declared"));
    }

    #[test]
    fn test_query_tag_not_associated_with_all_events() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
userIdTag = Tag{userId String}

userRegistered = Event<userIdTag>{
    id String
    name String
}

other = Event{
    hello String
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered{timestamp Int*}]
    query [
        {
            eventTypes [userRegistered, other],
            tags [userIdTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(
            errors.iter().any(|e| e.message.contains("userIdTag") && e.message.contains("not associated")),
            "expected error for tag not associated with event: {:?}",
            errors
        );
    }

    #[test]
    fn test_valid_tag_associations() {
        let meta = get_meta();
        let schema = parse_schema(
            r#"
commonTag = Tag{x String}

userRegistered = Event<commonTag>{
    id String
    name String
}

other = Event<commonTag>{
    hello String
}

registerUser = Command{
    fields {
        id String
        name String
    }
    emits [userRegistered{timestamp Int*}]
    query [
        {
            eventTypes [userRegistered, other],
            tags [commonTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    fn get_meta_with_constraint() -> Vec<MetaBlock> {
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
    @constraint forall(tags, t => forall(eventTypes, e => e.assoc(t)))
}

command {
    fields {* _ Type}

    @source [fields]
    emits []event

    query []queryItem
}
"#;
        extract_blocks(&parse_meta(input).unwrap())
    }

    #[test]
    fn test_constraint_valid() {
        let meta = get_meta_with_constraint();
        let schema = parse_schema(
            r#"
commonTag = Tag{x String}

event1 = Event<commonTag>{
    a String
}

event2 = Event<commonTag>{
    b String
}

cmd = Command{
    fields {
        x String
    }
    query [
        {
            eventTypes [event1, event2],
            tags [commonTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(errors.is_empty(), "unexpected constraint errors: {:?}", errors);
    }

    #[test]
    fn test_constraint_violation() {
        let meta = get_meta_with_constraint();
        // event2 doesn't have myTag, so constraint should fail
        let schema = parse_schema(
            r#"
myTag = Tag{x String}

event1 = Event<myTag>{
    a String
}

event2 = Event{
    b String
}

cmd = Command{
    fields {
        x String
    }
    emits [event1{timestamp Int*}]
    query [
        {
            eventTypes [event1, event2],
            tags [myTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        // Should have constraint violation (and possibly tag association error)
        assert!(
            errors.iter().any(|e| e.message.contains("constraint")),
            "expected constraint violation error: {:?}",
            errors
        );
    }

    #[test]
    fn test_constraint_exists() {
        // Test exists quantifier
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
    @constraint exists(tags, t => forall(eventTypes, e => e.assoc(t)))
}

command {
    query []queryItem
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // At least one tag (commonTag) is associated with all events
        let schema = parse_schema(
            r#"
commonTag = Tag{x String}
otherTag = Tag{y String}

event1 = Event<commonTag>{
    a String
}

event2 = Event<commonTag>{
    b String
}

cmd = Command{
    query [
        {
            eventTypes [event1, event2],
            tags [commonTag, otherTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        // Should pass because commonTag satisfies the exists constraint
        let constraint_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("constraint")).collect();
        assert!(constraint_errors.is_empty(), "unexpected constraint errors: {:?}", constraint_errors);
    }

    #[test]
    fn test_constraint_not() {
        // Test NOT operator
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
    @constraint !exists(tags, t => !forall(eventTypes, e => e.assoc(t)))
}

command {
    query []queryItem
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // This is equivalent to forall(tags, t => forall(eventTypes, e => e.assoc(t)))
        let schema = parse_schema(
            r#"
commonTag = Tag{x String}

event1 = Event<commonTag>{
    a String
}

event2 = Event<commonTag>{
    b String
}

cmd = Command{
    query [
        {
            eventTypes [event1, event2],
            tags [commonTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        let constraint_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("constraint")).collect();
        assert!(constraint_errors.is_empty(), "unexpected constraint errors: {:?}", constraint_errors);
    }

    #[test]
    fn test_source_for_paths() {
        // Test @source [fields] for [tags] - only tags field should be validated
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
}

command {
    fields {* _ Type}

    @source [fields] for [tags]
    query []queryItem
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // tags need sourcing, eventTypes don't
        // myTag has field 'x' which matches fields.x
        // event1 has field 'a' which does NOT match fields - but that's OK because eventTypes is not in for[]
        let schema = parse_schema(
            r#"
myTag = Tag{x String}

event1 = Event{
    a String
}

cmd = Command{
    fields {
        x String
    }
    query [
        {
            eventTypes [event1],
            tags [myTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        let source_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("not found in source")).collect();
        assert!(source_errors.is_empty(), "unexpected source errors: {:?}", source_errors);
    }

    #[test]
    fn test_source_for_paths_missing() {
        // Test @source [fields] for [tags] - tags field SHOULD fail if field missing
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
}

command {
    fields {* _ Type}

    @source [fields] for [tags]
    query []queryItem
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // myTag has field 'missing' which is NOT in fields
        let schema = parse_schema(
            r#"
myTag = Tag{missing String}

event1 = Event{
    a String
}

cmd = Command{
    fields {
        x String
    }
    query [
        {
            eventTypes [event1],
            tags [myTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        let source_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("not found in source")).collect();
        assert!(!source_errors.is_empty(), "expected source error for missing field");
        assert!(source_errors[0].message.contains("missing"), "error should mention 'missing' field");
    }

    #[test]
    fn test_source_without_for_validates_all() {
        // Test @source [fields] (without for) - all refs should be validated
        let input = r#"
event {
    _ Type
}

tag {1 _ Type}

queryItem {
    eventTypes []event
    tags []tag
}

command {
    fields {* _ Type}

    @source [fields]
    query []queryItem
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // Without for[], both eventTypes and tags need validation
        // event1 has 'a' and myTag has 'x' - neither matches fields.y
        let schema = parse_schema(
            r#"
myTag = Tag{x String}

event1 = Event{
    a String
}

cmd = Command{
    fields {
        y String
    }
    query [
        {
            eventTypes [event1],
            tags [myTag]
        }
    ]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        let source_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("not found in source")).collect();
        // Should fail because 'a' and 'x' are not in fields
        assert!(!source_errors.is_empty(), "expected source errors for missing fields");
    }

    #[test]
    fn test_type_expects_pascal_case() {
        let input = r#"
tag {1 _ Type}
"#;
        let parsed = parse_meta(input).unwrap();
        let meta = extract_blocks(&parsed);
        let type_defs = extract_type_defs(&parsed);

        // lowercase 'string' should fail - Type expects PascalCase
        let schema = parse_schema(
            r#"
badTag = Tag{userId string}
"#,
        )
        .unwrap();

        let errors = validate_sources_with_type_defs(&meta, &type_defs, &schema);
        assert!(
            errors.iter().any(|e| e.message.contains("PascalCase")),
            "expected PascalCase error: {:?}",
            errors
        );
    }

    #[test]
    fn test_type_accepts_pascal_case() {
        let input = r#"
tag {1 _ Type}
"#;
        let parsed = parse_meta(input).unwrap();
        let meta = extract_blocks(&parsed);
        let type_defs = extract_type_defs(&parsed);

        // PascalCase should pass
        let schema = parse_schema(
            r#"
goodTag = Tag{userId String}
"#,
        )
        .unwrap();

        let errors = validate_sources_with_type_defs(&meta, &type_defs, &schema);
        let type_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("PascalCase")).collect();
        assert!(type_errors.is_empty(), "unexpected type errors: {:?}", type_errors);
    }

    #[test]
    fn test_concrete_string_expects_literal() {
        let input = r#"
tag {1 _ Type} | Concrete<String>
"#;
        let parsed = parse_meta(input).unwrap();
        let meta = extract_blocks(&parsed);
        let type_defs = extract_type_defs(&parsed);

        // Object form with Type should pass
        let schema1 = parse_schema(
            r#"
objectTag = Tag{userId String}
"#,
        )
        .unwrap();

        let errors1 = validate_sources_with_type_defs(&meta, &type_defs, &schema1);
        let type_errors1: Vec<_> = errors1.iter().filter(|e| e.message.contains("concrete") || e.message.contains("union")).collect();
        assert!(type_errors1.is_empty(), "object form should pass: {:?}", type_errors1);
    }

    #[test]
    fn test_concrete_string_rejects_type_ref() {
        let input = r#"
config {
    label Concrete<String>
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // Type reference should fail for Concrete<String>
        let schema = parse_schema(
            r#"
myConfig = Config{
    label String
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        assert!(
            errors.iter().any(|e| e.message.contains("concrete string")),
            "expected concrete string error: {:?}",
            errors
        );
    }

    #[test]
    fn test_concrete_string_accepts_literal() {
        let input = r#"
config {
    label Concrete<String>
}
"#;
        let meta = extract_blocks(&parse_meta(input).unwrap());

        // Literal string should pass
        let schema = parse_schema(
            r#"
myConfig = Config{
    label "My Label"
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&meta, &schema);
        let type_errors: Vec<_> = errors.iter().filter(|e| e.message.contains("concrete")).collect();
        assert!(type_errors.is_empty(), "literal string should pass: {:?}", type_errors);
    }
}
