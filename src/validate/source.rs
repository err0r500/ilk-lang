use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::S;
use crate::validate::structural::{get_field_type_from_type_expr, ValidationContext};

pub fn validate_source(ctx: &ValidationContext, inst: &Instance) -> Vec<Diagnostic> {
    let mut errors = Vec::new();
    let type_name = &inst.type_name.node;
    if let Some(meta_decl) = ctx.env.get_meta(type_name) {
        validate_instance_sources(ctx, inst, &meta_decl.node, &mut errors);
    }
    errors
}

fn validate_instance_sources(
    ctx: &ValidationContext,
    inst: &Instance,
    meta_decl: &MetaDecl,
    errors: &mut Vec<Diagnostic>,
) {
    if let Value::Struct(inst_fields) = &inst.body.node {
        validate_struct_sources(ctx, inst_fields, meta_decl, errors);
    }
}

fn validate_struct_sources(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    meta_decl: &MetaDecl,
    errors: &mut Vec<Diagnostic>,
) {
    let type_fields = match &meta_decl.body.node {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => f.as_slice(),
        TypeExpr::Intersection(left, right) => {
            let mut all_fields = Vec::new();
            collect_struct_fields(&left.node, &mut all_fields);
            collect_struct_fields(&right.node, &mut all_fields);
            validate_fields_sources_inner(ctx, inst_fields, &all_fields, errors);
            return;
        }
        _ => return,
    };

    let refs: Vec<&S<Field>> = type_fields.iter().collect();
    validate_fields_sources_inner(ctx, inst_fields, &refs, errors);
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

fn validate_fields_sources_inner(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[&S<Field>],
    errors: &mut Vec<Diagnostic>,
) {
    for type_field in type_fields {
        let field_name = &type_field.node.name.node;

        let source_ann = type_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            if let Some(inst_field) = inst_fields.iter().find(|f| &f.node.name.node == field_name) {
                validate_field_source(
                    ctx,
                    inst_field,
                    sources,
                    inst_fields,
                    &type_field.node.ty.node,
                    errors,
                );
            }
        }

        // Recurse into nested struct values when the field meta is a named type
        if let Some(inst_field) = inst_fields.iter().find(|f| &f.node.name.node == field_name) {
            if let Value::Struct(nested_inst_fields) = &inst_field.node.value.node {
                if let Some(nested_type_name) =
                    get_type_name_from_type_expr(&type_field.node.ty.node)
                {
                    if let Some(nested_meta_decl) = ctx.env.get_meta(&nested_type_name) {
                        validate_struct_sources(
                            ctx,
                            nested_inst_fields,
                            &nested_meta_decl.node,
                            errors,
                        );
                    }
                }
            }
        }
    }
}

/// Check if a meta is a list of references (e.g., []&Event)
fn is_reference_list(ty: &TypeExpr) -> bool {
    if let TypeExpr::List(_, inner) = ty {
        matches!(inner.node, TypeExpr::Reference(_))
    } else {
        false
    }
}

fn resolve_nested_type<'a>(
    ctx: &'a ValidationContext,
    parent_type: Option<&'a TypeExpr>,
    name: &str,
) -> Option<&'a TypeExpr> {
    parent_type
        .and_then(|pt| get_field_type_from_type_expr(ctx, pt, name))
        .map(|t| &t.node)
}

fn validate_ref_instance_fields(
    ctx: &ValidationContext,
    ref_name: &str,
    caller_span: crate::span::Span,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    parent_type: Option<&TypeExpr>,
    skip: &std::collections::HashSet<&str>,
    errors: &mut Vec<Diagnostic>,
) {
    if let Some(ref_inst) = ctx.get_instance(ref_name) {
        let ref_ctx = ctx.for_instance(ref_name);
        if let Value::Struct(ref_fields) = &ref_inst.body.node {
            for ref_field in ref_fields {
                if !skip.is_empty() && skip.contains(ref_field.node.name.node.as_str()) {
                    continue;
                }
                // Collect errors separately so we can re-attribute them to the
                // caller's file/span rather than the referenced instance's.
                let mut ref_errors = Vec::new();
                validate_refinement_field(
                    &ref_ctx,
                    ref_field,
                    sources,
                    parent_fields,
                    parent_type,
                    &mut ref_errors,
                );
                for mut err in ref_errors {
                    err.file = ctx.path.to_path_buf();
                    err.span = caller_span.clone();
                    errors.push(err);
                }
            }
        }
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
    if let Value::List(elements) = &inst_field.node.value.node {
        let elem_type = match field_type {
            TypeExpr::List(_, inner) => Some(&inner.node),
            _ => None,
        };
        validate_list_elements(ctx, elements, sources, parent_fields, elem_type, errors);
    } else if let Value::Struct(nested_fields) = &inst_field.node.value.node {
        let fields_to_skip = get_fields_to_skip(field_type, ctx);
        for nested in nested_fields {
            if fields_to_skip.contains(&nested.node.name.node) {
                if let Value::Struct(inner_fields) = &nested.node.value.node {
                    for inner in inner_fields {
                        validate_refinement_field(ctx, inner, sources, parent_fields, None, errors);
                    }
                }
                continue;
            }
            validate_refinement_field(
                ctx,
                nested,
                sources,
                parent_fields,
                Some(field_type),
                errors,
            );
        }
    } else if let Value::BindingRef(ref_name) | Value::Refinement(ref_name, _) =
        &inst_field.node.value.node
    {
        validate_binding_ref_or_refinement(
            ctx,
            inst_field,
            ref_name,
            sources,
            parent_fields,
            field_type,
            errors,
        );
    }
}

fn validate_list_elements(
    ctx: &ValidationContext,
    elements: &[S<ListElement>],
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    elem_type: Option<&TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    let no_skip = std::collections::HashSet::new();
    for elem in elements {
        match &elem.node {
            ListElement::Refinement(_name, ref_fields) => {
                for ref_field in ref_fields {
                    validate_refinement_field(
                        ctx,
                        ref_field,
                        sources,
                        parent_fields,
                        elem_type,
                        errors,
                    );
                }
            }
            ListElement::BindingRef(name) => {
                validate_ref_instance_fields(
                    ctx,
                    name,
                    elem.span.clone(),
                    sources,
                    parent_fields,
                    elem_type,
                    &no_skip,
                    errors,
                );
            }
            ListElement::Value(v) => match v {
                Value::Struct(fields) => {
                    for field in fields {
                        validate_refinement_field(
                            ctx,
                            field,
                            sources,
                            parent_fields,
                            elem_type,
                            errors,
                        );
                    }
                }
                Value::BindingRef(name) => {
                    validate_ref_instance_fields(
                        ctx,
                        name,
                        elem.span.clone(),
                        sources,
                        parent_fields,
                        elem_type,
                        &no_skip,
                        errors,
                    );
                }
                _ => {}
            },
        }
    }
}

fn validate_binding_ref_or_refinement(
    ctx: &ValidationContext,
    inst_field: &S<InstanceField>,
    ref_name: &str,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    field_type: &TypeExpr,
    errors: &mut Vec<Diagnostic>,
) {
    let ref_fields: &[S<InstanceField>] = match &inst_field.node.value.node {
        Value::Refinement(_, f) => f,
        _ => &[],
    };

    for field in ref_fields {
        validate_refinement_field(ctx, field, sources, parent_fields, Some(field_type), errors);
    }

    let fields_to_skip = get_fields_to_skip(field_type, ctx);
    let refined_names: std::collections::HashSet<&str> = ref_fields
        .iter()
        .map(|f| f.node.name.node.as_str())
        .collect();
    let skip: std::collections::HashSet<&str> = refined_names
        .iter()
        .copied()
        .chain(fields_to_skip.iter().map(|s| s.as_str()))
        .collect();
    validate_ref_instance_fields(
        ctx,
        ref_name,
        inst_field.span.clone(),
        sources,
        parent_fields,
        Some(field_type),
        &skip,
        errors,
    );
}

/// Get field names that should be skipped from parent @source validation
fn get_fields_to_skip(ty: &TypeExpr, ctx: &ValidationContext) -> std::collections::HashSet<String> {
    let mut result = std::collections::HashSet::new();
    collect_fields_to_skip_resolved(ty, ctx, &mut result);
    result
}

fn collect_fields_to_skip_resolved(
    ty: &TypeExpr,
    ctx: &ValidationContext,
    result: &mut std::collections::HashSet<String>,
) {
    match ty {
        TypeExpr::Named(name) | TypeExpr::RefinableRef(name) => {
            if let Some(decl) = ctx.env.get_meta(name) {
                collect_fields_to_skip(&decl.node.body.node, result);
            }
        }
        TypeExpr::Union(variants) => {
            for variant in variants {
                collect_fields_to_skip_resolved(&variant.node, ctx, result);
            }
        }
        other => collect_fields_to_skip(other, result),
    }
}

fn collect_fields_to_skip(ty: &TypeExpr, result: &mut std::collections::HashSet<String>) {
    match ty {
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            for field in fields {
                for ann in &field.node.annotations {
                    if let Annotation::Source(paths) = &ann.node {
                        for path in paths {
                            result.insert(path.node.root_name().to_owned());
                        }
                    }
                }
            }

            for field in fields {
                let has_source = field
                    .node
                    .annotations
                    .iter()
                    .any(|a| matches!(a.node, Annotation::Source(_)));
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
    parent_type: Option<&TypeExpr>,
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
            if let Value::Struct(nested_fields) = &field.node.value.node {
                validate_none_origin_struct(
                    ctx,
                    name,
                    nested_fields,
                    sources,
                    parent_fields,
                    parent_type,
                    errors,
                );
                return;
            }
            if let Value::Refinement(ref_name, ref_fields) = &field.node.value.node {
                validate_none_origin_refinement(
                    ctx,
                    name,
                    ref_name,
                    ref_fields,
                    field.span.clone(),
                    sources,
                    parent_fields,
                    parent_type,
                    errors,
                );
                return;
            }
            if let Value::List(elements) = &field.node.value.node {
                validate_none_origin_list(
                    ctx,
                    name,
                    elements,
                    sources,
                    parent_fields,
                    parent_type,
                    errors,
                );
                return;
            }

            // Exempt &T / -T reference fields from parent @source validation
            if let Value::BindingRef(_) = &field.node.value.node {
                let is_ref_type = resolve_nested_type(ctx, parent_type, name)
                    .map(|t| matches!(t, TypeExpr::Reference(_) | TypeExpr::RefinableRef(_)))
                    .unwrap_or(false);
                if is_ref_type {
                    return;
                }
            }

            let found = check_implicit_source(ctx, field, sources, parent_fields, errors);
            if !found && !is_concrete_value(&field.node.value) {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!("No source found for field '{}'", name),
                    ctx.path,
                ));
            }
        }
    }
}

fn validate_none_origin_struct(
    ctx: &ValidationContext,
    name: &str,
    nested_fields: &[S<InstanceField>],
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    parent_type: Option<&TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    let nested_type = resolve_nested_type(ctx, parent_type, name);
    for nested in nested_fields {
        validate_refinement_field(ctx, nested, sources, parent_fields, nested_type, errors);
    }
}

fn validate_none_origin_refinement(
    ctx: &ValidationContext,
    name: &str,
    ref_name: &str,
    ref_fields: &[S<InstanceField>],
    caller_span: crate::span::Span,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    parent_type: Option<&TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    let nested_type = resolve_nested_type(ctx, parent_type, name);
    for ref_field in ref_fields {
        validate_refinement_field(ctx, ref_field, sources, parent_fields, nested_type, errors);
    }
    let refined_names: std::collections::HashSet<&str> = ref_fields
        .iter()
        .map(|f| f.node.name.node.as_str())
        .collect();
    validate_ref_instance_fields(
        ctx,
        ref_name,
        caller_span,
        sources,
        parent_fields,
        nested_type,
        &refined_names,
        errors,
    );
}

fn validate_none_origin_list(
    ctx: &ValidationContext,
    name: &str,
    elements: &[S<ListElement>],
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    parent_type: Option<&TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    let field_type_expr = resolve_nested_type(ctx, parent_type, name);
    let is_ref_list = field_type_expr
        .map(|t| is_reference_list(t))
        .unwrap_or(true);

    let elem_type = field_type_expr.and_then(|t| match t {
        TypeExpr::List(_, inner) => Some(&inner.node),
        _ => None,
    });

    let no_skip = std::collections::HashSet::new();
    for elem in elements {
        match &elem.node {
            ListElement::BindingRef(ref_name) => {
                if is_ref_list {
                    continue;
                }
                validate_ref_instance_fields(
                    ctx,
                    ref_name,
                    elem.span.clone(),
                    sources,
                    parent_fields,
                    elem_type,
                    &no_skip,
                    errors,
                );
            }
            ListElement::Refinement(_, ref_fields) => {
                for ref_field in ref_fields {
                    validate_refinement_field(
                        ctx,
                        ref_field,
                        sources,
                        parent_fields,
                        elem_type,
                        errors,
                    );
                }
            }
            ListElement::Value(Value::Struct(fields)) => {
                for f in fields {
                    validate_refinement_field(ctx, f, sources, parent_fields, elem_type, errors);
                }
            }
            ListElement::Value(Value::BindingRef(ref_name)) => {
                if is_ref_list {
                    continue;
                }
                validate_ref_instance_fields(
                    ctx,
                    ref_name,
                    elem.span.clone(),
                    sources,
                    parent_fields,
                    elem_type,
                    &no_skip,
                    errors,
                );
            }
            _ => {}
        }
    }
}

fn source_allows_path(sources: &[S<SourcePath>], path: &[String]) -> bool {
    sources.iter().any(|s| {
        let source_parts: Vec<&str> = match &s.node {
            SourcePath::Simple(name) => vec![name.as_str()],
            SourcePath::Dotted(parts) => parts.iter().map(|s| s.as_str()).collect(),
        };
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
    let mut source_optional = false;

    for (i, segment) in path.iter().enumerate() {
        if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
            if f.node.optional {
                source_optional = true;
            }
            if i < path.len() - 1 {
                let nested = match &f.node.value.node {
                    Value::Struct(fields) => Some(fields.as_slice()),
                    Value::BindingRef(name) | Value::Refinement(name, _) => ctx
                        .get_instance(name)
                        .and_then(|inst| match &inst.body.node {
                            Value::Struct(fields) => Some(fields.as_slice()),
                            _ => None,
                        }),
                    _ => None,
                };
                match nested {
                    Some(fields) => current_fields = fields,
                    None => {
                        errors.push(Diagnostic::error(
                            field.span.clone(),
                            format!("Source path '{}' not found", path.join(".")),
                            ctx.path,
                        ));
                        return;
                    }
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

    if !field.node.optional && source_optional {
        errors.push(Diagnostic::error(
            field.span.clone(),
            format!(
                "Mandatory field cannot depend on optional source '{}'",
                path.join(".")
            ),
            ctx.path,
        ));
    }

    if let Some(src) = source_field {
        let src_type = get_value_type(&src.node.value.node);
        let dst_type = get_value_type(&field.node.value.node);
        if let (Some(s), Some(d)) = (src_type, dst_type) {
            if s != d {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!(
                        "Type mismatch: source '{}' is {} but field is {}",
                        path.join("."),
                        s,
                        d
                    ),
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
    ctx: &ValidationContext,
    field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    errors: &mut Vec<Diagnostic>,
) -> bool {
    let field_name = &field.node.name.node;

    let matches: Vec<(&str, &S<InstanceField>)> = sources
        .iter()
        .filter_map(|source| {
            let root = source.node.root_name();
            let source_path_parts: Vec<&str> = match &source.node {
                SourcePath::Simple(name) => vec![name.as_str()],
                SourcePath::Dotted(parts) => parts.iter().map(|s| s.as_str()).collect(),
            };
            let mut current_fields: &[S<InstanceField>] = parent_fields;
            for part in &source_path_parts {
                let f = current_fields.iter().find(|f| f.node.name.node == *part)?;
                match &f.node.value.node {
                    Value::Struct(fields) => current_fields = fields,
                    Value::BindingRef(name) | Value::Refinement(name, _) => {
                        let inst = ctx.get_instance(name)?;
                        match &inst.body.node {
                            Value::Struct(fields) => current_fields = fields,
                            _ => return None,
                        }
                    }
                    _ => return None,
                }
            }
            let src_field = current_fields
                .iter()
                .find(|f| &f.node.name.node == field_name)?;
            Some((root, src_field))
        })
        .collect();

    match matches.len() {
        0 => false,
        1 => {
            let (root, src_field) = matches[0];

            if !field.node.optional && src_field.node.optional {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!(
                        "Mandatory field cannot depend on optional source '{}.{}'",
                        root, field_name
                    ),
                    ctx.path,
                ));
            }

            let src_type = get_value_type(&src_field.node.value.node);
            let dst_type = get_value_type(&field.node.value.node);
            if let (Some(s), Some(d)) = (src_type, dst_type) {
                if s != d {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!(
                            "Type mismatch: source '{}.{}' is {} but field is {}",
                            root, field_name, s, d
                        ),
                        ctx.path,
                    ));
                }
            }
            true
        }
        _ => {
            let sources_list: Vec<_> = matches.iter().map(|(r, _)| *r).collect();
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!(
                    "Ambiguous source for '{}': found in [{}]. Use explicit mapping.",
                    field_name,
                    sources_list.join(", ")
                ),
                ctx.path,
            ));
            true
        }
    }
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
        let env = resolve(&file, Path::new("test.ilk")).0;
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        for inst in file.instances() {
            errors.extend(validate_structural(&ctx, inst));
            errors.extend(validate_source(&ctx, inst));
        }
        errors
    }

    #[test]
    fn test_generated_exempt() {
        let errors = validate_source_src(
            r#"
meta Event = {timestamp Int}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}

cmd = Cmd {
  fields {x Int}
  emits {timestamp Int*}
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_list_generated_exempt() {
        let errors = validate_source_src(
            r#"
meta Event = {timestamp Int}
meta Cmd = {
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
meta Event = {id String}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
cmd = Cmd {
  fields {userId String}
  emits {id String = fields.userId}
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_mapped_valid_list() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
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
meta Event = {id String}
meta Cmd = {
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
    fn test_source_not_found_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
cmd = Cmd {
  fields {x Int}
  emits {id String = other.id}
}
"#,
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_concrete_exempt() {
        let errors = validate_source_src(
            r#"
meta Event = {status Concrete<Int>}
meta Cmd = {
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
    fn test_concrete_exempt_single() {
        let errors = validate_source_src(
            r#"
meta Event = {status Concrete<Int>}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
cmd = Cmd {
  fields {x Int}
  emits {status 200}
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_binding_ref_missing_source() {
        let errors = validate_source_src(
            r#"
meta Event = {bla String}
meta Cmd = {
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
        assert!(errors[0]
            .message
            .contains("No source found for field 'bla'"));
    }

    #[test]
    fn test_binding_ref_missing_source_single() {
        let errors = validate_source_src(
            r#"
meta Event = {bla String}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
other = Event {bla String}
cmd = Cmd {
  fields {x Int}
  emits other
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("No source found for field 'bla'"));
    }

    #[test]
    fn test_binding_ref_valid_source() {
        let errors = validate_source_src(
            r#"
meta Event = {bla String}
meta Cmd = {
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
    fn test_binding_ref_valid_source_single() {
        let errors = validate_source_src(
            r#"
meta Event = {bla String}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
other = Event {bla String}
cmd = Cmd {
  fields {bla String}
  emits other
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_mapped_type_mismatch() {
        let errors = validate_source_src(
            r#"
meta Event = {id Int}
meta Cmd = {
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
    fn test_mapped_type_mismatch_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id Int}
meta Cmd = {
  fields {...}
  @source [fields]
  emits Event
}
cmd = Cmd {
  fields {userId Uuid}
  emits {id Int = fields.userId}
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
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Command = {
  fields {...}
  @source [fields]
  emits []Event
}
meta Wrapper = {
  outerFields {...}
  @source [outerFields]
  command Command
}
ev = Event {userId String}
wrapper = Wrapper {
  outerFields {userId String}
  command {
    fields {userId String}
    emits [ev]
  }
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_nested_struct_source_missing_in_local_context() {
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Command = {
  fields {...}
  @source [fields]
  emits []Event
}
meta Wrapper = {
  outerFields {...}
  @source [outerFields]
  command Command
}
ev = Event {userId String}
wrapper = Wrapper {
  outerFields {userId String, x Int}
  command {
    fields {x Int}
    emits [ev]
  }
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("No source found for field 'userId'")),
            "Expected 'No source found for field userId', got: {:?}",
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_source_path_prefix_validation() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Endpoint = {
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
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_source_path_prefix_validation_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Endpoint = {
    method String
    params {...}
    body {...}
    @source [params, body]
    response Event
}
endpoint = Endpoint {
    method String
    params {id String}
    body {}
    response {id String = params.id}
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_source_path_rejects_outside_subtree() {
        let errors = validate_source_src(
            r#"
meta Event = {method String}
meta Endpoint = {
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
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("not allowed by @source"));
    }

    #[test]
    fn test_source_path_rejects_outside_subtree_single() {
        let errors = validate_source_src(
            r#"
meta Event = {method String}
meta Endpoint = {
    method String
    params {...}
    body {...}
    @source [params, body]
    response Event
}
endpoint = Endpoint {
    method String
    params {x Int}
    body {}
    response {method String = method}
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("not allowed by @source"));
    }

    #[test]
    fn test_mandatory_from_optional_source() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! []Event
}
e = Event {id String}
cmd = Cmd {
  fields {opt? String}
  emits [e & {id String = fields.opt}]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("Mandatory field cannot depend on optional source"));
    }

    #[test]
    fn test_mandatory_from_optional_source_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! Event
}
cmd = Cmd {
  fields {opt? String}
  emits {id String = fields.opt}
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0]
            .message
            .contains("Mandatory field cannot depend on optional source"));
    }

    #[test]
    fn test_optional_from_optional_source() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! []Event
}
e = Event {id String}
cmd = Cmd {
  fields {opt? String}
  emits [e & {id? String = fields.opt}]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_optional_from_optional_source_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id String}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! Event
}
cmd = Cmd {
  fields {opt? String}
  emits {id? String = fields.opt}
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_optional_from_optional_type_mismatch() {
        let errors = validate_source_src(
            r#"
meta Event = {id Int}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! []Event
}
e = Event {id Int}
cmd = Cmd {
  fields {opt? String}
  emits [e & {id? Int = fields.opt}]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
    }

    #[test]
    fn test_optional_from_optional_type_mismatch_single() {
        let errors = validate_source_src(
            r#"
meta Event = {id Int}
meta Cmd = {
  fields! {...}
  @source [fields]
  emits! Event
}
cmd = Cmd {
  fields {opt? String}
  emits {id? Int = fields.opt}
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
    }

    #[test]
    fn test_implicit_type_mismatch() {
        let errors = validate_source_src(
            r#"
meta Event = {userId Uuid}
meta Cmd = {
  fields {...}
  auth {...}
  @source [fields, auth]
  emits []Event
}
e = Event {userId Uuid}
cmd = Cmd {
  fields {x Int}
  auth {userId String}
  emits [e]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
        assert!(errors[0].message.contains("auth.userId"));
    }

    #[test]
    fn test_implicit_type_mismatch_single() {
        let errors = validate_source_src(
            r#"
meta Event = {userId Uuid}
meta Cmd = {
  fields {...}
  auth {...}
  @source [fields, auth]
  emits Event
}
other = Event {userId Uuid}
cmd = Cmd {
  fields {x Int}
  auth {userId String}
  emits other
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
        assert!(errors[0].message.contains("auth.userId"));
    }

    #[test]
    fn test_ambiguous_source() {
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Cmd = {
  fields {...}
  auth {...}
  @source [fields, auth]
  emits []Event
}
e = Event {userId String}
cmd = Cmd {
  fields {userId String}
  auth {userId String}
  emits [e]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Ambiguous source"));
        assert!(errors[0].message.contains("fields"));
        assert!(errors[0].message.contains("auth"));
    }

    #[test]
    fn test_ambiguous_source_single() {
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Cmd = {
  fields {...}
  auth {...}
  @source [fields, auth]
  emits Event
}
other = Event {userId String}
cmd = Cmd {
  fields {userId String}
  auth {userId String}
  emits other
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Ambiguous source"));
        assert!(errors[0].message.contains("fields"));
        assert!(errors[0].message.contains("auth"));
    }

    #[test]
    fn test_mapped_in_source_root_field_validates_path() {
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Command = {
  fields {...}
  @source [fields]
  emits []Event
}
meta Endpoint = {
    method String
    params {...}
    body {...}
}
meta Slice = {
    endpoint Endpoint
    @source [endpoint.params, endpoint.body]
    command Command
}
ev = Event {userId String}
slice = Slice {
    endpoint {
        method "POST"
        params {cartId Uuid}
        body {description String}
    }
    command {
        fields {
            id Uuid = endpoint.params.id
            name String = endpoint.body.name
        }
        emits [ev & {userId String = fields.id}]
    }
}
"#,
        );
        assert!(
            !errors.is_empty(),
            "Expected errors for invalid source paths in source root fields"
        );
        let messages: Vec<&str> = errors.iter().map(|e| e.message.as_str()).collect();
        assert!(
            messages.iter().any(|m| m.contains("not found")),
            "Expected 'not found' error, got: {:?}",
            messages
        );
    }

    #[test]
    fn test_explicit_source_on_ref_list_valid() {
        let errors = validate_source_src(
            r#"
meta Tag = {name String}
meta Event = {
    fields {...}
    @source [fields]
    tags []&Tag
}
t = Tag {name String}
ev = Event {
    fields {name String}
    tags [t]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_explicit_source_on_ref_list_missing() {
        let errors = validate_source_src(
            r#"
meta Tag = {name String}
meta Event = {
    fields {...}
    @source [fields]
    tags []&Tag
}
t = Tag {name String}
ev = Event {
    fields {x Int}
    tags [t]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("No source found for field 'name'")),
            "Expected 'No source found for field name', got: {:?}",
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_implicit_in_source_root_field_validates() {
        let errors = validate_source_src(
            r#"
meta Event = {userId String}
meta Command = {
  fields {...}
  @source [fields]
  emits []Event
}
meta Endpoint = {
    method String
    params {...}
    body {...}
}
meta Slice = {
    endpoint Endpoint
    @source [endpoint.params, endpoint.body]
    command Command
}
ev = Event {userId String}
slice = Slice {
    endpoint {
        method "POST"
        params {cartId Uuid}
        body {description String}
    }
    command {
        fields {
            cartId Uuid
            description String
            shopperId String
        }
        emits [ev & {userId String = fields.cartId}]
    }
}
"#,
        );
        assert!(
            !errors.is_empty(),
            "Expected error for unmatched field 'shopperId'"
        );
        assert!(
            errors.iter().any(|e| e.message.contains("shopperId")),
            "Expected error about 'shopperId', got: {:?}",
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_union_field_type_skips_own_source() {
        // When a field is typed as a union of named types, fields with their own
        // @source in those types should be skipped from parent @source validation.
        let errors = validate_source_src(
            r#"
meta Schema = {...}
meta QueryA = {
    name! Concrete<String>
    params {...}
    @source [params]
    result {...}
}
meta QueryB = {
    name! Concrete<String>
    params {...}
    @source [params]
    result {...}
}
meta Wrapper = {
    auth {...}
    @source [auth]
    query QueryA | QueryB
}
qa = QueryA {
    name "qa"
    params {userId Uuid}
    result {id Uuid = params.userId}
}
w = Wrapper {
    auth {userId Uuid}
    query qa
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_union_field_type_still_validates_non_sourced_fields() {
        // Fields without their own @source in union variants, and not referenced
        // as source roots, should still be validated against parent @source.
        let errors = validate_source_src(
            r#"
meta QueryA = {
    name! Concrete<String>
    extra {...}
    params {...}
    @source [params]
    result {...}
}
meta Wrapper = {
    auth {...}
    @source [auth]
    query QueryA
}
qa = QueryA {
    name "qa"
    extra {x Int}
    params {y Uuid}
    result {y Uuid = params.y}
}
w = Wrapper {
    auth {z String}
    query qa
}
"#,
        );
        assert!(
            !errors.is_empty(),
            "Expected error: extra.x has no match in auth"
        );
        assert!(
            errors.iter().any(|e| e.message.contains("No source found")),
            "Expected 'No source found' error, got: {:?}",
            errors.iter().map(|e| &e.message).collect::<Vec<_>>()
        );
    }

    #[test]
    fn test_ref_field_exempt_from_parent_source() {
        // &T binding ref fields should be exempt from parent @source validation
        // since they are references/configuration, not data flow.
        let errors = validate_source_src(
            r#"
meta Schema = {...}
meta Query = {
    table &Schema
    params {...}
    @source [table]
    return {...}
}
meta Wrapper = {
    auth {...}
    @source [auth]
    query Query
}
tbl = Schema {id Uuid, name String}
q = Query {
    table tbl
    params {userId Uuid}
    return {id Uuid = table.id}
}
w = Wrapper {
    auth {userId Uuid}
    query q
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_refinable_ref_field_exempt_from_parent_source() {
        // -T refinable ref fields should also be exempt from parent @source validation.
        let errors = validate_source_src(
            r#"
meta Schema = {...}
meta Command = {
    params {...}
    table &Schema
    @source [params]
    insert -Schema
    return {...}
}
meta Wrapper = {
    auth {...}
    @source [auth]
    command Command
}
tbl = Schema {id Uuid, name String}
cmd = Command {
    params {name String}
    table tbl
    insert tbl & {
        id Uuid*
        name String = params.name
    }
    return {id Uuid*}
}
w = Wrapper {
    auth {name String}
    command cmd
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }
}
