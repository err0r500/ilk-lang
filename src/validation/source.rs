use crate::ast::{RawBlock, RawItem, RawValue, Span, TypeRefinement};
use crate::parser::schema::{SchemaBlock, SchemaField, SchemaItem};
use crate::schema::Annotation;
use std::collections::HashMap;
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

/// Validates @source annotations on instance blocks against schema.
/// For fields with @source [sources], validates that emitted event fields
/// exist in the source blocks with matching types (unless marked generated).
pub fn validate_sources(schema: &[SchemaBlock], instance: &[RawBlock]) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    // Build lookup for schema blocks
    let schema_map: HashMap<&str, &SchemaBlock> = schema
        .iter()
        .map(|b| (b.kind.value().as_str(), b))
        .collect();

    // Build lookup for instance blocks (for event definitions)
    let instance_map: HashMap<&str, &RawBlock> = instance
        .iter()
        .filter_map(|b| b.name.as_ref().map(|n| (n.value().as_str(), b)))
        .collect();

    for block in instance {
        if let Some(schema_def) = schema_map.get(block.kind.value().as_str()) {
            validate_block(schema_def, block, &instance_map, &mut errors);
        }
    }

    errors
}

fn validate_block(
    schema_def: &SchemaBlock,
    block: &RawBlock,
    instance_map: &HashMap<&str, &RawBlock>,
    errors: &mut Vec<ValidationError>,
) {
    // Collect source fields from this block
    let source_fields = collect_source_fields(schema_def, block);

    for item in &block.body {
        if let RawItem::Field(field) = &item.0 {
            // Check if this field has @source annotation in schema
            if let Some(schema_field) = find_schema_field(schema_def, &field.name.0) {
                if let Some(_sources) = get_source_annotation(&schema_field.annotations) {
                    // Field value should be a list of emitted events
                    if let RawValue::List(emits) = &field.value.0 {
                        for emit in emits {
                            validate_emitted_event(emit, &source_fields, instance_map, errors);
                        }
                    }
                }
            }
        }
    }
}

fn find_schema_field<'a>(schema_def: &'a SchemaBlock, name: &str) -> Option<&'a SchemaField> {
    for item in &schema_def.body {
        if let SchemaItem::Field(f) = &item.0 {
            // Match by name or wildcard
            if f.name.value() == name || f.name.value() == "_" {
                return Some(f);
            }
        }
    }
    None
}

fn get_source_annotation(annotations: &[Annotation]) -> Option<&Vec<String>> {
    annotations.iter().find_map(|a| match a {
        Annotation::Source(sources) => Some(sources),
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
fn collect_source_fields(schema_def: &SchemaBlock, block: &RawBlock) -> HashMap<String, FieldInfo> {
    let mut fields = HashMap::new();

    // Find which nested blocks are sources from schema
    let source_names: Vec<&str> = schema_def
        .body
        .iter()
        .filter_map(|item| {
            if let SchemaItem::Field(f) = &item.0 {
                get_source_annotation(&f.annotations).map(|srcs| srcs.iter().map(|s| s.as_str()))
            } else {
                None
            }
        })
        .flatten()
        .collect();

    // Find source data in instance - can be either:
    // 1. Nested Block with matching kind name
    // 2. Field with Object value and matching field name (parser ambiguity)
    for item in &block.body {
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
    for item in &block.body {
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

fn validate_emitted_event(
    emit: &crate::ast::Spanned<RawValue>,
    source_fields: &HashMap<String, FieldInfo>,
    instance_map: &HashMap<&str, &RawBlock>,
    errors: &mut Vec<ValidationError>,
) {
    match &emit.0 {
        // Simple ref: emits [UserRegistered] - need all fields from event def
        RawValue::Ref(parts) => {
            let event_name = parts.join(".");
            if let Some(event_block) = instance_map.get(event_name.as_str()) {
                validate_event_fields_covered(
                    event_block,
                    None,
                    source_fields,
                    emit.span(),
                    errors,
                );
            }
        }
        // TypeRefinement: emits [UserRegistered{timestamp int*}]
        RawValue::TypeRefinement(refinement) => {
            if let Some(event_block) = instance_map.get(refinement.base.as_str()) {
                validate_event_fields_covered(
                    event_block,
                    Some(refinement),
                    source_fields,
                    emit.span(),
                    errors,
                );
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

fn validate_event_fields_covered(
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
    for item in &event_block.body {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::instance::parse_instance;
    use crate::parser::schema::parse_schema;

    fn get_schema() -> Vec<SchemaBlock> {
        let input = r#"
event {
    _ type
}

command {
    fields {_ type}

    @source [fields]
    emits []event
}
"#;
        parse_schema(input).unwrap()
    }

    #[test]
    fn test_valid_emits_with_generated() {
        let schema = get_schema();
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
    timestamp int
}

command RegisterUser {
    fields {
        id string
        name string
    }
    emits [UserRegistered{timestamp int*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_invalid_emits_missing_field() {
        let schema = get_schema();
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
    timestamp int
}

command RegisterUser {
    fields {
        id string
        name string
    }
    emits [UserRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(!errors.is_empty(), "expected validation error");
        assert!(errors[0].message.contains("timestamp"));
    }

    #[test]
    fn test_optional_field_without_source_ok() {
        let schema = get_schema();
        // Optional field WITHOUT source should PASS (it's optional)
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
    timestamp? int
}

command RegisterUser {
    fields {
        id string
        name string
    }
    emits [UserRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(
            errors.is_empty(),
            "optional field without source should pass"
        );
    }

    #[test]
    fn test_optional_field_with_source_ok() {
        let schema = get_schema();
        // Optional field WITH matching source field should PASS
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
    timestamp? int
}

command RegisterUser {
    fields {
        id string
        name string
        timestamp int
    }
    emits [UserRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_optional_field_with_generated_ok() {
        let schema = get_schema();
        // Optional field marked as generated should PASS
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
    timestamp? int
}

command RegisterUser {
    fields {
        id string
        name string
    }
    emits [UserRegistered{timestamp int*}]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(errors.is_empty(), "unexpected errors: {:?}", errors);
    }

    #[test]
    fn test_type_mismatch() {
        let schema = get_schema();
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
}

command RegisterUser {
    fields {
        id int
        name string
    }
    emits [UserRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(!errors.is_empty(), "expected type mismatch error");
        assert!(errors[0].message.contains("type mismatch"));
    }

    #[test]
    fn test_required_field_from_optional_source_fails() {
        let schema = get_schema();
        // Required target field sourced from optional source field should FAIL
        let instance = parse_instance(
            r#"
event UserRegistered {
    id string
    name string
}

command RegisterUser {
    fields {
        id string
        name? string
    }
    emits [UserRegistered]
}
"#,
        )
        .unwrap();

        let errors = validate_sources(&schema, &instance);
        assert!(
            !errors.is_empty(),
            "expected error for required field from optional source"
        );
        assert!(
            errors[0].message.contains("required field") && errors[0].message.contains("optional")
        );
    }
}
