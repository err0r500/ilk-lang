use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::S;
use crate::validate::structural::{ValidationContext, get_field_type_from_type_expr};

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
        // Pass instance-level assocs as initial context
        validate_struct_sources_with_assocs(ctx, inst_fields, type_decl, &inst.assocs, errors);
    }
}

fn validate_struct_sources_with_assocs(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_decl: &TypeDecl,
    parent_assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    // Get the struct fields from the type body
    let type_fields = match &type_decl.body.node {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => f.as_slice(),
        TypeExpr::Intersection(left, right) => {
            let mut all_fields = Vec::new();
            collect_struct_fields(&left.node, &mut all_fields);
            collect_struct_fields(&right.node, &mut all_fields);
            validate_intersection_struct_sources_with_assocs(ctx, inst_fields, &all_fields, parent_assocs, errors);
            return;
        }
        _ => return,
    };

    validate_type_fields_sources_with_assocs(ctx, inst_fields, type_fields, parent_assocs, errors);
}

fn validate_type_fields_sources_with_assocs(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[S<Field>],
    parent_assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    let refs: Vec<&S<Field>> = type_fields.iter().collect();
    validate_fields_sources_inner_with_assocs(ctx, inst_fields, &refs, parent_assocs, errors);
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

fn validate_intersection_struct_sources_with_assocs(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[&S<Field>],
    parent_assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    validate_fields_sources_inner_with_assocs(ctx, inst_fields, type_fields, parent_assocs, errors);
}

fn validate_fields_sources_inner_with_assocs(
    ctx: &ValidationContext,
    inst_fields: &[S<InstanceField>],
    type_fields: &[&S<Field>],
    parent_assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    for type_field in type_fields {
        let field_name = &type_field.node.name.node;

        let source_ann = type_field.node.annotations.iter().find_map(|a| match &a.node {
            Annotation::Source(paths) => Some(paths),
            _ => None,
        });

        if let Some(sources) = source_ann {
            if let Some(inst_field) = inst_fields.iter().find(|f| &f.node.name.node == field_name) {
                // Use field's own assocs if present, otherwise use parent assocs
                let active_assocs = if inst_field.node.assocs.is_empty() {
                    parent_assocs
                } else {
                    &inst_field.node.assocs
                };
                validate_field_source_with_assocs(ctx, inst_field, sources, inst_fields, &type_field.node.ty.node, active_assocs, errors);
            }
        }

        // Recurse into nested struct values when the field type is a named type
        if let Some(inst_field) = inst_fields.iter().find(|f| &f.node.name.node == field_name) {
            if let Value::Struct(nested_inst_fields) = &inst_field.node.value.node {
                if let Some(nested_type_name) = get_type_name_from_type_expr(&type_field.node.ty.node) {
                    if let Some(nested_type_decl) = ctx.env.get_type(&nested_type_name) {
                        // Use field's assocs if present, otherwise pass parent assocs
                        let nested_assocs = if inst_field.node.assocs.is_empty() {
                            parent_assocs
                        } else {
                            &inst_field.node.assocs
                        };
                        validate_struct_sources_with_assocs(ctx, nested_inst_fields, &nested_type_decl.node, nested_assocs, errors);
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

fn validate_field_source_with_assocs(
    ctx: &ValidationContext,
    inst_field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    field_type: &TypeExpr,
    parent_assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    // Get assocs: field's own assocs take precedence over parent assocs
    let assocs: &[S<String>] = if !inst_field.node.assocs.is_empty() {
        &inst_field.node.assocs
    } else {
        parent_assocs
    };

    // Only validate lists (refinements and inline elements)
    // Nested struct values are handled by recursive validate_struct_sources
    if let Value::List(elements) = &inst_field.node.value.node {
        // For reference lists ([]&Type), skip validation of plain binding refs
        // since references don't carry data - only validate refinements
        let is_ref_list = is_reference_list(field_type);

        // Get the element type for list types
        let elem_type = match field_type {
            TypeExpr::List(_, inner) => Some(&inner.node),
            _ => None,
        };

        for elem in elements {
            match &elem.node {
                ListElement::Refinement(_name, elem_assocs, ref_fields) => {
                    // Use element-level assocs if present, otherwise field-level
                    let active_assocs = if elem_assocs.is_empty() { assocs } else { elem_assocs };
                    // Refinements introduce inline modifications that need source validation
                    for ref_field in ref_fields {
                        validate_refinement_field(ctx, ref_field, sources, parent_fields, active_assocs, elem_type, errors);
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
                                validate_refinement_field(ctx, ref_field, sources, parent_fields, assocs, elem_type, errors);
                            }
                        }
                    }
                }
                ListElement::Value(v) => match v {
                    Value::Struct(fields) => {
                        // Inline structs need source validation
                        for field in fields {
                            validate_refinement_field(ctx, field, sources, parent_fields, assocs, elem_type, errors);
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
                                        ctx, ref_field, sources, parent_fields, assocs, elem_type, errors,
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
            // Skip fields with own @source or @out annotation - they're validated separately
            if fields_to_skip.contains(&nested.node.name.node) {
                continue;
            }
            // Use nested field's assocs if present, otherwise use parent assocs
            let nested_assocs = if nested.node.assocs.is_empty() { assocs } else { &nested.node.assocs };
            validate_refinement_field(ctx, nested, sources, parent_fields, nested_assocs, Some(field_type), errors);
        }
    } else if let Value::BindingRef(ref_name) | Value::Refinement(ref_name, _, _) = &inst_field.node.value.node {
        // Extract refinement info if present
        let (ref_assocs, ref_fields): (&[S<String>], &[S<InstanceField>]) = match &inst_field.node.value.node {
            Value::Refinement(_, a, f) => (a, f),
            _ => (&[], &[]),
        };
        let active_assocs = if ref_assocs.is_empty() { assocs } else { ref_assocs };

        // Validate explicit refinement fields first
        for field in ref_fields {
            validate_refinement_field(ctx, field, sources, parent_fields, active_assocs, Some(field_type), errors);
        }

        // Validate non-refined fields from referenced instance
        if let Some(ref_inst) = ctx.get_instance(ref_name) {
            if let Value::Struct(inst_fields) = &ref_inst.body.node {
                let fields_to_skip = get_fields_to_skip(field_type, ctx);
                let refined_names: std::collections::HashSet<_> = ref_fields.iter().map(|f| &f.node.name.node).collect();
                for nested in inst_fields {
                    if refined_names.contains(&nested.node.name.node) || fields_to_skip.contains(&nested.node.name.node) {
                        continue;
                    }
                    let nested_assocs = if nested.node.assocs.is_empty() { active_assocs } else { &nested.node.assocs };
                    validate_refinement_field(ctx, nested, sources, parent_fields, nested_assocs, Some(field_type), errors);
                }
            }
        }
    }
}

/// Get field names that should be skipped from parent @source validation
/// This includes fields with their own @source annotation, and open struct fields (declarations)
fn get_fields_to_skip(ty: &TypeExpr, ctx: &ValidationContext) -> std::collections::HashSet<String> {
    let mut result = std::collections::HashSet::new();
    let body = if let TypeExpr::Named(name) = ty {
        ctx.env.get_type(name).map(|t| &t.node.body.node)
    } else {
        Some(ty)
    };
    if let Some(body) = body {
        collect_fields_to_skip(body, &mut result);
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
                            // Skip fields that ARE sources (input declarations)
                            result.insert(path.node.root_name().to_owned());
                        }
                    }
                }
            }

            // Second pass: skip fields with their own @source annotation OR @out annotation
            for field in fields {
                let has_source = field.node.annotations.iter().any(|a| {
                    matches!(a.node, Annotation::Source(_))
                });
                let has_out = field.node.annotations.iter().any(|a| {
                    matches!(a.node, Annotation::Out)
                });
                if has_source || has_out {
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
    assocs: &[S<String>],
    parent_type: Option<&TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    let name = &field.node.name.node;
    // Use field's own assocs if present, otherwise use passed-in assocs
    let active_assocs = if field.node.assocs.is_empty() { assocs } else { &field.node.assocs };

    match &field.node.origin {
        FieldOrigin::Generated => {
            // Exempt - no check needed
        }
        FieldOrigin::Mapped(path) => {
            if !source_allows_path(sources, path, active_assocs) {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!("Source path '{}' not allowed by @source", path.join(".")),
                    ctx.path,
                ));
            } else {
                validate_source_path(ctx, path, field, parent_fields, active_assocs, errors);
            }
        }
        FieldOrigin::Computed(paths) => {
            for path in paths {
                if !source_allows_path(sources, path, active_assocs) {
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
                let nested_type = parent_type
                    .and_then(|pt| get_field_type_from_type_expr(ctx, pt, name))
                    .map(|t| &t.node);
                for nested in nested_fields {
                    validate_refinement_field(ctx, nested, sources, parent_fields, active_assocs, nested_type, errors);
                }
                // Don't error on parent - child errors are sufficient
                return;
            }

            // If this is a refinement, validate its fields
            if let Value::Refinement(ref_name, ref_assocs, ref_fields) = &field.node.value.node {
                let nested_type = parent_type
                    .and_then(|pt| get_field_type_from_type_expr(ctx, pt, name))
                    .map(|t| &t.node);
                let ref_active_assocs = if ref_assocs.is_empty() { active_assocs } else { ref_assocs };
                for ref_field in ref_fields {
                    validate_refinement_field(ctx, ref_field, sources, parent_fields, ref_active_assocs, nested_type, errors);
                }
                // Also validate non-refined fields from referenced instance
                if let Some(ref_inst) = ctx.get_instance(ref_name) {
                    if let Value::Struct(inst_fields) = &ref_inst.body.node {
                        let refined_names: std::collections::HashSet<_> = ref_fields.iter().map(|f| &f.node.name.node).collect();
                        for nested in inst_fields {
                            if refined_names.contains(&nested.node.name.node) {
                                continue;
                            }
                            let nested_assocs = if nested.node.assocs.is_empty() { ref_active_assocs } else { &nested.node.assocs };
                            validate_refinement_field(ctx, nested, sources, parent_fields, nested_assocs, nested_type, errors);
                        }
                    }
                }
                return;
            }

            // If this is a list, validate its elements recursively
            if let Value::List(elements) = &field.node.value.node {
                // Resolve the field's type to check if it's a reference list
                let field_type_expr = parent_type
                    .and_then(|pt| get_field_type_from_type_expr(ctx, pt, name))
                    .map(|t| &t.node);
                let is_ref_list = field_type_expr.map(|t| is_reference_list(t)).unwrap_or(true);

                // For list types, get the element type
                let elem_type = field_type_expr.and_then(|t| match t {
                    TypeExpr::List(_, inner) => Some(&inner.node),
                    _ => None,
                });

                for elem in elements {
                    match &elem.node {
                        ListElement::BindingRef(ref_name) => {
                            if is_ref_list {
                                continue;
                            }
                            // Value list: validate referenced instance's fields
                            if let Some(ref_inst) = ctx.get_instance(ref_name) {
                                if let Value::Struct(ref_fields) = &ref_inst.body.node {
                                    for ref_field in ref_fields {
                                        validate_refinement_field(ctx, ref_field, sources, parent_fields, active_assocs, elem_type, errors);
                                    }
                                }
                            }
                        }
                        ListElement::Refinement(_, elem_assocs, ref_fields) => {
                            let ref_assocs = if elem_assocs.is_empty() { active_assocs } else { elem_assocs };
                            for ref_field in ref_fields {
                                validate_refinement_field(ctx, ref_field, sources, parent_fields, ref_assocs, elem_type, errors);
                            }
                        }
                        ListElement::Value(Value::Struct(fields)) => {
                            for f in fields {
                                validate_refinement_field(ctx, f, sources, parent_fields, active_assocs, elem_type, errors);
                            }
                        }
                        ListElement::Value(Value::BindingRef(ref_name)) => {
                            if is_ref_list {
                                continue;
                            }
                            // Value list: validate referenced instance's fields
                            if let Some(ref_inst) = ctx.get_instance(ref_name) {
                                if let Value::Struct(ref_fields) = &ref_inst.body.node {
                                    for ref_field in ref_fields {
                                        validate_refinement_field(ctx, ref_field, sources, parent_fields, active_assocs, elem_type, errors);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
                return;
            }

            let found = check_implicit_source(ctx, field, sources, parent_fields, active_assocs, errors);
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
/// Also handles $assoc paths when assocs context is available
fn source_allows_path(sources: &[S<SourcePath>], path: &[String], assocs: &[S<String>]) -> bool {
    // Handle $assoc paths: if path starts with $assoc and we have @source [assoc],
    // check that the instance name is in the assocs list
    if path.first().map(|s| s.as_str()) == Some("$assoc") {
        // Check if @source includes "assoc"
        let has_assoc_source = sources.iter().any(|s| {
            matches!(&s.node, SourcePath::Simple(name) if name == "assoc")
        });
        if has_assoc_source {
            // Check the instance name (path[1]) is in assocs
            if let Some(inst_name) = path.get(1) {
                return assocs.iter().any(|a| &a.node == inst_name);
            }
        }
        return false;
    }

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
    assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    // Handle $assoc paths: $assoc.instanceName.fieldPath
    if path.first().map(|s| s.as_str()) == Some("$assoc") {
        validate_assoc_source_path(ctx, path, field, assocs, errors);
        return;
    }

    let mut current_fields = parent_fields;
    let mut source_field: Option<&S<InstanceField>> = None;
    let mut source_optional = false;

    for (i, segment) in path.iter().enumerate() {
        if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
            // Track if any field in the path is optional
            if f.node.optional {
                source_optional = true;
            }
            if i < path.len() - 1 {
                // Get nested fields from struct, binding ref, or refinement
                let nested = match &f.node.value.node {
                    Value::Struct(fields) => Some(fields.as_slice()),
                    Value::BindingRef(name) | Value::Refinement(name, _, _) => {
                        ctx.get_instance(name).and_then(|inst| match &inst.body.node {
                            Value::Struct(fields) => Some(fields.as_slice()),
                            _ => None,
                        })
                    }
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

    // Check optionality: mandatory field cannot depend on optional source
    if !field.node.optional && source_optional {
        errors.push(Diagnostic::error(
            field.span.clone(),
            format!("Mandatory field cannot depend on optional source '{}'", path.join(".")),
            ctx.path,
        ));
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

/// Validate $assoc paths: $assoc.instanceName.fieldPath
fn validate_assoc_source_path(
    ctx: &ValidationContext,
    path: &[String],
    field: &S<InstanceField>,
    assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) {
    // path = ["$assoc", "instanceName", "fieldName", ...]
    if path.len() < 3 {
        errors.push(Diagnostic::error(
            field.span.clone(),
            format!("Invalid $assoc path '{}': expected $assoc.instanceName.field", path.join(".")),
            ctx.path,
        ));
        return;
    }

    let inst_name = &path[1];

    // Check instance name is in assocs
    if !assocs.iter().any(|a| &a.node == inst_name) {
        errors.push(Diagnostic::error(
            field.span.clone(),
            format!("Instance '{}' not in inline assocs", inst_name),
            ctx.path,
        ));
        return;
    }

    // Get the instance
    let inst = match ctx.get_instance(inst_name) {
        Some(i) => i,
        None => {
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!("Unknown instance in $assoc path: {}", inst_name),
                ctx.path,
            ));
            return;
        }
    };

    // Navigate through instance body for remaining path segments
    let remaining_path = &path[2..];
    let inst_fields = match &inst.body.node {
        Value::Struct(fields) => fields,
        _ => {
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!("Instance '{}' is not a struct", inst_name),
                ctx.path,
            ));
            return;
        }
    };

    let mut current_fields: &[S<InstanceField>] = inst_fields;
    let mut source_field: Option<&S<InstanceField>> = None;
    let mut source_optional = false;

    for (i, segment) in remaining_path.iter().enumerate() {
        if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
            if f.node.optional {
                source_optional = true;
            }
            if i < remaining_path.len() - 1 {
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
                format!("Field '{}' not found in instance '{}'", segment, inst_name),
                ctx.path,
            ));
            return;
        }
    }

    // Check optionality
    if !field.node.optional && source_optional {
        errors.push(Diagnostic::error(
            field.span.clone(),
            format!("Mandatory field cannot depend on optional source '{}'", path.join(".")),
            ctx.path,
        ));
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
    ctx: &ValidationContext,
    field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    parent_fields: &[S<InstanceField>],
    assocs: &[S<String>],
    errors: &mut Vec<Diagnostic>,
) -> bool {
    let field_name = &field.node.name.node;

    // Collect all matching sources from parent fields
    let matches: Vec<(&str, &S<InstanceField>)> = sources
        .iter()
        .filter_map(|source| {
            let root = source.node.root_name();
            // Skip "assoc" source - handled separately below
            if root == "assoc" {
                return None;
            }
            let source_field = parent_fields.iter().find(|f| f.node.name.node == root)?;
            if let Value::Struct(source_fields) = &source_field.node.value.node {
                let src_field = source_fields
                    .iter()
                    .find(|f| &f.node.name.node == field_name)?;
                Some((root, src_field))
            } else {
                None
            }
        })
        .collect();

    // Check if @source includes "assoc" - if so, also search assoc instances
    let has_assoc_source = sources.iter().any(|s| {
        matches!(&s.node, SourcePath::Simple(name) if name == "assoc")
    });

    // Store assoc matches separately (can't mix lifetimes with parent_fields)
    let assoc_matches: Vec<(&str, &S<InstanceField>)> = if has_assoc_source {
        assocs
            .iter()
            .filter_map(|assoc_name| {
                let inst = ctx.get_instance(&assoc_name.node)?;
                if let Value::Struct(inst_fields) = &inst.body.node {
                    let src_field = inst_fields
                        .iter()
                        .find(|f| &f.node.name.node == field_name)?;
                    Some((assoc_name.node.as_str(), src_field))
                } else {
                    None
                }
            })
            .collect()
    } else {
        vec![]
    };

    // Combine matches
    let total_matches = matches.len() + assoc_matches.len();

    match total_matches {
        0 => false,
        1 => {
            // Get the single match from either matches or assoc_matches
            let (root, src_field) = if !matches.is_empty() {
                matches[0]
            } else {
                assoc_matches[0]
            };

            // Check optionality: mandatory field cannot depend on optional source
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

            // Check type compatibility
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
            // Combine all source names for the error message
            let mut sources_list: Vec<_> = matches.iter().map(|(r, _)| *r).collect();
            sources_list.extend(assoc_matches.iter().map(|(r, _)| *r));
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!(
                    "Ambiguous source for '{}': found in [{}]. Use explicit mapping.",
                    field_name,
                    sources_list.join(", ")
                ),
                ctx.path,
            ));
            true // Return true to avoid additional "no source found" error
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

    #[test]
    fn test_mandatory_from_optional_source() {
        let errors = validate_source_src(
            r#"
type Event = {id String}
type Cmd = {
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
        assert!(errors[0].message.contains("Mandatory field cannot depend on optional source"));
    }

    #[test]
    fn test_optional_from_optional_source() {
        let errors = validate_source_src(
            r#"
type Event = {id String}
type Cmd = {
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
        // Should pass: optional field can depend on optional source
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_optional_from_optional_type_mismatch() {
        let errors = validate_source_src(
            r#"
type Event = {id Int}
type Cmd = {
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
        // Should fail: type mismatch even if both optional
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Type mismatch"));
    }

    #[test]
    fn test_implicit_type_mismatch() {
        let errors = validate_source_src(
            r#"
type Event = {userId Uuid}
type Cmd = {
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
    fn test_ambiguous_source() {
        let errors = validate_source_src(
            r#"
type Event = {userId String}
type Cmd = {
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
    fn test_assoc_source_valid() {
        let errors = validate_source_src(
            r#"
type TableSchema = {...}
@assoc [TableSchema]
type DbQuery = {
    funcName Concrete<String>
    @source [assoc]
    @out
    return {...}
}
userTable = TableSchema {
    id Uuid
    name String
}
query = DbQuery {
    funcName "test"
    return <userTable> {
        id Uuid = $assoc.userTable.id
        name String = $assoc.userTable.name
    }
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_assoc_source_not_in_assocs() {
        let errors = validate_source_src(
            r#"
type TableSchema = {...}
@assoc [TableSchema]
type DbQuery = {
    @source [assoc]
    @out
    return {...}
}
userTable = TableSchema {name String}
otherTable = TableSchema {name String}
query = DbQuery {
    return <userTable> {
        name String = $assoc.otherTable.name
    }
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("not allowed by @source") || errors[0].message.contains("not in inline assocs"));
    }

    #[test]
    fn test_assoc_nested_field_with_inline_assocs() {
        let errors = validate_source_src(
            r#"
type TableSchema = {...}
@assoc [TableSchema]
type DbQuery = {
    @source [assoc]
    @out
    return {...}
}
type Endpoint = {
    query DbQuery
}
userTable = TableSchema {id Uuid}
endpoint = Endpoint {
    query <userTable> {
        return {
            id Uuid = $assoc.userTable.id
        }
    }
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }
}
