use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::S;
use crate::validate::structural::ValidationContext;

pub fn validate_source(ctx: &ValidationContext, file: &File, errors: &mut Vec<Diagnostic>) {
    for inst in file.instances() {
        let type_name = &inst.type_name.node;
        if let Some(type_decl) = ctx.env.get_type(type_name) {
            validate_instance_sources(ctx, inst, &type_decl.node, errors);
        }
    }
}

fn validate_instance_sources(
    ctx: &ValidationContext,
    inst: &Instance,
    type_decl: &TypeDecl,
    errors: &mut Vec<Diagnostic>,
) {
    if let Value::Struct(inst_fields) = &inst.body.node {
        validate_struct_sources(ctx, inst_fields, type_decl, errors);
    }
}

fn validate_struct_sources(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_decl: &TypeDecl,
    errors: &mut Vec<Diagnostic>,
) {
    // Get the struct fields from the type body
    let type_fields = match &type_decl.body.node {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => f.as_slice(),
        TypeExpr::Intersection(left, right) => {
            let mut all_fields = Vec::new();
            collect_struct_fields(&left.node, &mut all_fields);
            collect_struct_fields(&right.node, &mut all_fields);
            validate_intersection_struct_sources(ctx, inst_fields, &all_fields, errors);
            return;
        }
        _ => return,
    };

    validate_type_fields_sources(ctx, inst_fields, type_fields, errors);
}

fn validate_type_fields_sources(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[S<Field>],
    errors: &mut Vec<Diagnostic>,
) {
    // Check each field for @source annotation
    for type_field in type_fields {
        let source_ann = type_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            if let Some(inst_field) = inst_fields
                .iter()
                .find(|f| f.node.name.node == type_field.node.name.node)
            {
                validate_field_source(ctx, inst_field, sources, inst_fields, &type_field.node.ty.node, errors);
            }
        }

        // Recurse into nested struct values if the field type is a named type
        if let Some(inst_field) = inst_fields
            .iter()
            .find(|f| f.node.name.node == type_field.node.name.node)
        {
            if let Value::Struct(nested_inst_fields) = &inst_field.node.value.node {
                // Get the type name from the type field
                if let Some(nested_type_name) = get_type_name_from_type_expr(&type_field.node.ty.node) {
                    if let Some(nested_type_decl) = ctx.env.get_type(&nested_type_name) {
                        validate_struct_sources(ctx, nested_inst_fields, &nested_type_decl.node, errors);
                    }
                }
            }
        }
    }
}

fn get_type_name_from_type_expr(type_expr: &TypeExpr) -> Option<String> {
    match type_expr {
        TypeExpr::Named(name) => Some(name.clone()),
        _ => None,
    }
}

fn collect_struct_fields<'a>(ty: &'a TypeExpr, fields: &mut Vec<&'a S<Field>>) {
    match ty {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => {
            fields.extend(f.iter());
        }
        TypeExpr::Intersection(left, right) => {
            collect_struct_fields(&left.node, fields);
            collect_struct_fields(&right.node, fields);
        }
        _ => {}
    }
}

fn validate_intersection_struct_sources(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[&S<Field>],
    errors: &mut Vec<Diagnostic>,
) {
    for type_field in type_fields {
        let source_ann = type_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            if let Some(inst_field) = inst_fields
                .iter()
                .find(|f| f.node.name.node == type_field.node.name.node)
            {
                validate_field_source(ctx, inst_field, sources, inst_fields, &type_field.node.ty.node, errors);
            }
        }

        // Recurse into nested struct values if the field type is a named type
        if let Some(inst_field) = inst_fields
            .iter()
            .find(|f| f.node.name.node == type_field.node.name.node)
        {
            if let Value::Struct(nested_inst_fields) = &inst_field.node.value.node {
                if let Some(nested_type_name) = get_type_name_from_type_expr(&type_field.node.ty.node) {
                    if let Some(nested_type_decl) = ctx.env.get_type(&nested_type_name) {
                        validate_struct_sources(ctx, nested_inst_fields, &nested_type_decl.node, errors);
                    }
                }
            }
        }
    }
}

/// Check if a type is a list of references (e.g., []&Event)
fn is_reference_list(ty: &TypeExpr) -> bool {
    if let TypeExpr::List(_, inner) = ty {
        matches!(inner.node, TypeExpr::Reference(_))
    } else {
        false
    }
}

fn validate_field_source(
    ctx: &ValidationContext,
    inst_field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    field_type: &TypeExpr,
    errors: &mut Vec<Diagnostic>,
) {
    // Only validate lists (refinements and inline elements)
    // Nested struct values are handled by recursive validate_struct_sources
    if let Value::List(elements) = &inst_field.node.value.node {
        // For reference lists ([]&Type), skip validation of plain binding refs
        // since references don't carry data - only validate refinements
        let is_ref_list = is_reference_list(field_type);

        for elem in elements {
            match &elem.node {
                ListElement::Refinement(_name, ref_fields) => {
                    // Refinements introduce inline modifications that need source validation
                    for ref_field in ref_fields {
                        validate_refinement_field(ctx, ref_field, sources, parent_fields, errors);
                    }
                }
                ListElement::BindingRef(name) => {
                    // Skip validation for reference types - they're just pointers
                    if is_ref_list {
                        continue;
                    }
                    // For value types, validate the referenced instance's fields
                    if let Some(ref_inst) = ctx.get_instance(name) {
                        if let Value::Struct(ref_fields) = &ref_inst.body.node {
                            for ref_field in ref_fields {
                                validate_refinement_field(ctx, ref_field, sources, parent_fields, errors);
                            }
                        }
                    }
                }
                ListElement::Value(v) => match v {
                    Value::Struct(fields) => {
                        // Inline structs need source validation
                        for field in fields {
                            validate_refinement_field(ctx, field, sources, parent_fields, errors);
                        }
                    }
                    Value::BindingRef(name) => {
                        // Skip validation for reference types - they're just pointers
                        if is_ref_list {
                            continue;
                        }
                        // For value types, validate the referenced instance's fields
                        if let Some(ref_inst) = ctx.get_instance(name) {
                            if let Value::Struct(ref_fields) = &ref_inst.body.node {
                                for ref_field in ref_fields {
                                    validate_refinement_field(
                                        ctx, ref_field, sources, parent_fields, errors,
                                    );
                                }
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
    } else if let Value::Struct(nested_fields) = &inst_field.node.value.node {
        // Validate struct fields against parent @source
        // But skip fields that have their own @source in the nested type
        let fields_to_skip = get_fields_to_skip(field_type, ctx);

        for nested in nested_fields {
            // Skip fields with own @source or open struct type (declarations)
            if fields_to_skip.contains(&nested.node.name.node) {
                // Still validate mappings inside source fields against parent source
                if let Value::Struct(inner) = &nested.node.value.node {
                    for inner_field in inner {
                        // Only validate fields with explicit mappings - declarations should not
                        // be checked against parent source (they're source roots themselves)
                        match &inner_field.node.origin {
                            FieldOrigin::Mapped(_) | FieldOrigin::Computed(_) => {
                                validate_refinement_field(ctx, inner_field, sources, parent_fields, errors);
                            }
                            _ => {}
                        }
                    }
                }
                continue;
            }
            validate_refinement_field(ctx, nested, sources, parent_fields, errors);
        }
    }
}

/// Get field names that should be skipped from parent @source validation
/// This includes fields with their own @source annotation, and open struct fields (declarations)
fn get_fields_to_skip(ty: &TypeExpr, ctx: &ValidationContext) -> std::collections::HashSet<String> {
    let mut result = std::collections::HashSet::new();

    let type_decl = match ty {
        TypeExpr::Named(name) => ctx.env.get_type(name),
        _ => None,
    };

    if let Some(type_decl) = type_decl {
        collect_fields_to_skip(&type_decl.node.body.node, &mut result);
    }

    result
}

fn collect_fields_to_skip(ty: &TypeExpr, result: &mut std::collections::HashSet<String>) {
    match ty {
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            // First pass: collect all source roots referenced in @source annotations
            for field in fields {
                for ann in &field.node.annotations {
                    if let Annotation::Source(paths) = &ann.node {
                        for path in paths {
                            let root = match &path.node {
                                SourcePath::Simple(name) => name,
                                SourcePath::Dotted(parts) => parts.first().unwrap(),
                            };
                            // Skip fields that ARE sources (input declarations)
                            result.insert(root.clone());
                        }
                    }
                }
            }

            // Second pass: skip fields with their own @source annotation
            for field in fields {
                let has_source = field.node.annotations.iter().any(|a| {
                    matches!(a.node, Annotation::Source(_))
                });
                if has_source {
                    result.insert(field.node.name.node.clone());
                }
            }
        }
        TypeExpr::Intersection(left, right) => {
            collect_fields_to_skip(&left.node, result);
            collect_fields_to_skip(&right.node, result);
        }
        _ => {}
    }
}

fn validate_refinement_field(
    ctx: &ValidationContext,
    field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    errors: &mut Vec<Diagnostic>,
) {
    let name = &field.node.name.node;

    match &field.node.origin {
        FieldOrigin::Generated => {
            // Exempt - no check needed
        }
        FieldOrigin::Mapped(path) => {
            if !source_allows_path(sources, path) {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!("Source path '{}' not allowed by @source", path.join(".")),
                    ctx.path,
                ));
            } else {
                validate_source_path(ctx, path, field, parent_fields, errors);
            }
        }
        FieldOrigin::Computed(paths) => {
            for path in paths {
                if !source_allows_path(sources, path) {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!("Compute path '{}' not allowed by @source", path.join(".")),
                        ctx.path,
                    ));
                }
            }
        }
        FieldOrigin::None => {
            // If this is a struct, validate children recursively
            if let Value::Struct(nested_fields) = &field.node.value.node {
                for nested in nested_fields {
                    validate_refinement_field(ctx, nested, sources, parent_fields, errors);
                }
                // Don't error on parent - child errors are sufficient
                return;
            }

            // If this is a list, validate its elements recursively
            // Note: we skip binding refs here because we don't have type context
            // to know if this is a reference list ([]&Type). Reference lists should
            // not validate their elements. If this list has @source, it will be
            // validated properly by validate_field_source with type info.
            if let Value::List(elements) = &field.node.value.node {
                for elem in elements {
                    match &elem.node {
                        ListElement::BindingRef(_) => {
                            // Skip - no type context to know if this is a reference list
                        }
                        ListElement::Refinement(_, ref_fields) => {
                            for ref_field in ref_fields {
                                validate_refinement_field(ctx, ref_field, sources, parent_fields, errors);
                            }
                        }
                        ListElement::Value(Value::Struct(fields)) => {
                            for f in fields {
                                validate_refinement_field(ctx, f, sources, parent_fields, errors);
                            }
                        }
                        ListElement::Value(Value::BindingRef(_)) => {
                            // Skip - no type context to know if this is a reference list
                        }
                        _ => {}
                    }
                }
                return;
            }

            let found = check_implicit_source(ctx, name, sources, parent_fields);
            if !found {
                if !is_concrete_value(&field.node.value) {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!("No source found for field '{}'", name),
                        ctx.path,
                    ));
                }
            }
        }
    }
}

/// Check if any source path is a prefix of the given path
/// e.g., @source [endpoint.params] allows endpoint.params.id but not endpoint.method
fn source_allows_path(sources: &[S<SourcePath>], path: &[String]) -> bool {
    sources.iter().any(|s| {
        let source_parts: Vec<&str> = match &s.node {
            SourcePath::Simple(name) => vec![name.as_str()],
            SourcePath::Dotted(parts) => parts.iter().map(|s| s.as_str()).collect(),
        };
        // path must start with source_parts
        path.len() >= source_parts.len()
            && path.iter().zip(source_parts.iter()).all(|(p, s)| p == *s)
    })
}

fn validate_source_path(
    ctx: &ValidationContext,
    path: &[String],
    field: &S<InstanceField>,
    parent_fields: &[S<InstanceField>],
    errors: &mut Vec<Diagnostic>,
) {
    let mut current_fields = parent_fields;
    let mut source_field: Option<&S<InstanceField>> = None;

    for (i, segment) in path.iter().enumerate() {
        if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
            if i < path.len() - 1 {
                if let Value::Struct(nested) = &f.node.value.node {
                    current_fields = nested;
                } else {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!("Source path '{}' not found", path.join(".")),
                        ctx.path,
                    ));
                    return;
                }
            } else {
                source_field = Some(f);
            }
        } else {
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!("Source path '{}' not found", path.join(".")),
                ctx.path,
            ));
            return;
        }
    }

    // Check type compatibility
    if let Some(src) = source_field {
        let src_type = get_value_type(&src.node.value.node);
        let dst_type = get_value_type(&field.node.value.node);
        if let (Some(s), Some(d)) = (src_type, dst_type) {
            if s != d {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!("Type mismatch: source '{}' is {} but field is {}", path.join("."), s, d),
                    ctx.path,
                ));
            }
        }
    }
}

fn get_value_type(value: &Value) -> Option<&str> {
    match value {
        Value::TypeRef(t) => Some(t),
        Value::LitString(_) => Some("String"),
        Value::LitInt(_) => Some("Int"),
        Value::LitBool(_) => Some("Bool"),
        _ => None,
    }
}

fn check_implicit_source(
    _ctx: &ValidationContext,
    field_name: &str,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
) -> bool {
    for source in sources {
        let root = match &source.node {
            SourcePath::Simple(name) => name,
            SourcePath::Dotted(parts) => parts.first().unwrap(),
        };

        if let Some(source_field) = parent_fields.iter().find(|f| &f.node.name.node == root) {
            if let Value::Struct(source_fields) = &source_field.node.value.node {
                if source_fields
                    .iter()
                    .any(|f| &f.node.name.node == field_name)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn is_concrete_value(value: &S<Value>) -> bool {
    matches!(
        value.node,
        Value::LitString(_) | Value::LitInt(_) | Value::LitBool(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::resolve::resolve;
    use crate::validate::structural::validate_structural;
    use std::path::Path;

    fn validate_source_src(src: &str) -> Vec<Diagnostic> {
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        validate_structural(&ctx, &file, &mut errors);
        validate_source(&ctx, &file, &mut errors);
        errors
    }

    #[test]
    fn test_generated_exempt() {
        let errors = validate_source_src(
            r#"
type Event = {timestamp Int}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
e = Event {timestamp Int}
cmd = Cmd {
  fields {x Int}
  emits [e & {timestamp Int*}]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_mapped_valid() {
        let errors = validate_source_src(
            r#"
type Event = {id String}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
e = Event {id String}
cmd = Cmd {
  fields {userId String}
  emits [e & {id String = fields.userId}]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_source_not_found() {
        let errors = validate_source_src(
            r#"
type Event = {id String}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
e = Event {id String}
cmd = Cmd {
  fields {x Int}
  emits [e & {id String = other.id}]
}
"#,
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_concrete_exempt() {
        let errors = validate_source_src(
            r#"
type Event = {status Concrete<Int>}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
e = Event {status 200}
cmd = Cmd {
  fields {x Int}
  emits [e & {status 200}]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_binding_ref_missing_source() {
        let errors = validate_source_src(
            r#"
type Event = {bla String}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
other = Event {bla String}
cmd = Cmd {
  fields {x Int}
  emits [other]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("No source found for field 'bla'"));
    }

    #[test]
    fn test_binding_ref_valid_source() {
        let errors = validate_source_src(
            r#"
type Event = {bla String}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
other = Event {bla String}
cmd = Cmd {
  fields {bla String}
  emits [other]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_mapped_type_mismatch() {
        let errors = validate_source_src(
            r#"
type Event = {id Int}
type Cmd = {
  fields {...}
  @source [fields]
  emits []Event
}
e = Event {id Int}
cmd = Cmd {
  fields {userId Uuid}
  emits [e & {id Int = fields.userId}]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
        assert!(errors[0].message.contains("Uuid"));
        assert!(errors[0].message.contains("Int"));
    }

    #[test]
    fn test_nested_struct_source_context() {
        // When validating nested structs with @source, the source paths
        // should be resolved against the local struct context, not top-level
        let errors = validate_source_src(
            r#"
type Event = {userId String}
type Command = {
  fields {...}
  @source [fields]
  emits []Event
}
type Wrapper = {
  outerFields {...}
  @source [outerFields]
  command Command
}
ev = Event {userId String}
wrapper = Wrapper {
  outerFields {x Int}
  command {
    fields {userId String}
    emits [ev]
  }
}
"#,
        );
        // Should pass: ev.userId is found in command.fields, not wrapper.outerFields
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_nested_struct_source_missing_in_local_context() {
        // When nested struct has @source, it should check against local context
        let errors = validate_source_src(
            r#"
type Event = {userId String}
type Command = {
  fields {...}
  @source [fields]
  emits []Event
}
type Wrapper = {
  outerFields {...}
  @source [outerFields]
  command Command
}
ev = Event {userId String}
wrapper = Wrapper {
  outerFields {userId String}
  command {
    fields {x Int}
    emits [ev]
  }
}
"#,
        );
        // Should fail: ev.userId is NOT in command.fields (only has x)
        // Even though wrapper.outerFields has userId, that's the wrong context
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("No source found for field 'userId'"));
    }

    #[test]
    fn test_source_path_prefix_validation() {
        // @source [params, body] should allow params.id
        // but reject method (not a subtree of params or body)
        let errors = validate_source_src(
            r#"
type Event = {id String}
type Endpoint = {
    method String
    params {...}
    body {...}
    @source [params, body]
    responses []Event
}
ev = Event {id String}
endpoint = Endpoint {
    method String
    params {id String}
    body {}
    responses [ev & {id String = params.id}]
}
"#,
        );
        // Should pass: params.id is allowed by @source [params, body]
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_source_path_rejects_outside_subtree() {
        let errors = validate_source_src(
            r#"
type Event = {method String}
type Endpoint = {
    method String
    params {...}
    body {...}
    @source [params, body]
    responses []Event
}
ev = Event {method String}
endpoint = Endpoint {
    method String
    params {x Int}
    body {}
    responses [ev & {method String = method}]
}
"#,
        );
        // Should fail: method is not within params or body subtrees
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("not allowed by @source"));
    }
}
