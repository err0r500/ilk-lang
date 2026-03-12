use crate::error::Diagnostic;
use crate::ilk::ast::*;
use crate::kli::ast::*;
use crate::span::S;
use crate::validate::structural::ValidationContext;

pub fn validate_source(ctx: &ValidationContext, kli: &KliFile, errors: &mut Vec<Diagnostic>) {
    for binding in &kli.bindings {
        let type_name = &binding.node.type_name.node;
        if let Some(block) = ctx.env.get(type_name) {
            validate_binding_sources(ctx, binding, block, errors);
        }
    }
}

fn validate_binding_sources(
    ctx: &ValidationContext,
    binding: &S<Binding>,
    block: &S<Block>,
    errors: &mut Vec<Diagnostic>,
) {
    // Get the struct fields from the block body
    let fields = match &block.node.body.node {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => f,
        TypeExpr::Intersection(left, right) => {
            // Handle intersection - collect fields from both sides
            let mut all_fields = Vec::new();
            collect_struct_fields(&left.node, &mut all_fields);
            collect_struct_fields(&right.node, &mut all_fields);
            // For now, handle inline
            validate_intersection_sources(ctx, binding, &all_fields, errors);
            return;
        }
        _ => return,
    };

    // Check each field for @source annotation
    for ilk_field in fields {
        let source_ann = ilk_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            // Get the corresponding kli field
            if let KliValue::Struct(kli_fields) = &binding.node.body.node {
                if let Some(kli_field) = kli_fields
                    .iter()
                    .find(|f| f.node.name.node == ilk_field.node.name.node)
                {
                    validate_field_source(ctx, kli_field, sources, binding, errors);
                }
            }
        }
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

fn validate_intersection_sources(
    ctx: &ValidationContext,
    binding: &S<Binding>,
    ilk_fields: &[&S<Field>],
    errors: &mut Vec<Diagnostic>,
) {
    for ilk_field in ilk_fields {
        let source_ann = ilk_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            if let KliValue::Struct(kli_fields) = &binding.node.body.node {
                if let Some(kli_field) = kli_fields
                    .iter()
                    .find(|f| f.node.name.node == ilk_field.node.name.node)
                {
                    validate_field_source(ctx, kli_field, sources, binding, errors);
                }
            }
        }
    }
}

fn validate_field_source(
    ctx: &ValidationContext,
    kli_field: &S<KliField>,
    sources: &[S<SourcePath>],
    binding: &S<Binding>,
    errors: &mut Vec<Diagnostic>,
) {
    // Check if field is a list with refinements
    if let KliValue::List(elements) = &kli_field.node.value.node {
        for elem in elements {
            match &elem.node {
                KliListElement::Refinement(_name, ref_fields) => {
                    for ref_field in ref_fields {
                        validate_refinement_field(ctx, ref_field, sources, binding, errors);
                    }
                }
                KliListElement::BindingRef(name) => {
                    // Binding refs in @source lists need their fields checked
                    if let Some(_ref_binding) = ctx.bindings.get(name) {
                        // The binding itself satisfies the source constraint
                        // through its own field mappings
                    }
                }
                KliListElement::Value(v) => {
                    if let KliValue::Struct(fields) = v {
                        for field in fields {
                            validate_refinement_field(ctx, field, sources, binding, errors);
                        }
                    }
                }
            }
        }
    } else if let KliValue::Struct(nested_fields) = &kli_field.node.value.node {
        // Direct struct field with @source
        for nested in nested_fields {
            validate_refinement_field(ctx, nested, sources, binding, errors);
        }
    }
}

fn validate_refinement_field(
    ctx: &ValidationContext,
    field: &S<KliField>,
    sources: &[S<SourcePath>],
    binding: &S<Binding>,
    errors: &mut Vec<Diagnostic>,
) {
    let name = &field.node.name.node;

    // Check @out fields are exempt
    // (would need to look up in ilk schema)

    match &field.node.origin {
        FieldOrigin::Generated => {
            // Exempt - no check needed
        }
        FieldOrigin::Mapped(path) => {
            // Check path root is in sources
            if let Some(root) = path.first() {
                if !source_contains_root(sources, root) {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!("Source path root '{}' not in @source list", root),
                        ctx.path,
                    ));
                } else {
                    // Validate path exists and types match
                    validate_source_path(ctx, path, field, binding, errors);
                }
            }
        }
        FieldOrigin::Computed(paths) => {
            // Check all path roots are in sources
            for path in paths {
                if let Some(root) = path.first() {
                    if !source_contains_root(sources, root) {
                        errors.push(Diagnostic::error(
                            field.span.clone(),
                            format!("Compute path root '{}' not in @source list", root),
                            ctx.path,
                        ));
                    }
                }
            }
        }
        FieldOrigin::None => {
            // Implicit match - check if field name exists in any source
            let found = check_implicit_source(ctx, name, sources, binding);
            if !found {
                // Check if it's a Concrete/literal value (exempt)
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

fn source_contains_root(sources: &[S<SourcePath>], root: &str) -> bool {
    sources.iter().any(|s| match &s.node {
        SourcePath::Simple(name) => name == root,
        SourcePath::Dotted(parts) => parts.first().map(|p| p == root).unwrap_or(false),
    })
}

fn validate_source_path(
    ctx: &ValidationContext,
    path: &[String],
    field: &S<KliField>,
    binding: &S<Binding>,
    errors: &mut Vec<Diagnostic>,
) {
    // Walk the path to find the source field
    if let KliValue::Struct(binding_fields) = &binding.node.body.node {
        let mut current_fields = binding_fields.as_slice();
        let mut found = true;

        for (i, segment) in path.iter().enumerate() {
            if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
                if i < path.len() - 1 {
                    // More segments to go - need to descend into struct
                    if let KliValue::Struct(nested) = &f.node.value.node {
                        current_fields = nested;
                    } else {
                        found = false;
                        break;
                    }
                }
                // Last segment - check types would go here
            } else {
                found = false;
                break;
            }
        }

        if !found {
            errors.push(Diagnostic::error(
                field.span.clone(),
                format!("Source path '{}' not found", path.join(".")),
                ctx.path,
            ));
        }
    }
}

fn check_implicit_source(
    _ctx: &ValidationContext,
    field_name: &str,
    sources: &[S<SourcePath>],
    binding: &S<Binding>,
) -> bool {
    // Look for field_name in any of the source paths
    if let KliValue::Struct(binding_fields) = &binding.node.body.node {
        for source in sources {
            let root = match &source.node {
                SourcePath::Simple(name) => name,
                SourcePath::Dotted(parts) => parts.first().unwrap(),
            };

            // Find the source field in binding
            if let Some(source_field) = binding_fields.iter().find(|f| &f.node.name.node == root) {
                // Look for field_name in that source
                if let KliValue::Struct(source_fields) = &source_field.node.value.node {
                    if source_fields
                        .iter()
                        .any(|f| &f.node.name.node == field_name)
                    {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn is_concrete_value(value: &S<KliValue>) -> bool {
    matches!(
        value.node,
        KliValue::LitString(_) | KliValue::LitInt(_) | KliValue::LitBool(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ilk::{parse_ilk, resolve};
    use crate::kli::parse_kli;
    use crate::validate::structural::validate_structural;
    use std::path::Path;

    fn validate_source_pair(ilk_src: &str, kli_src: &str) -> Vec<Diagnostic> {
        let ilk = parse_ilk(ilk_src, Path::new("test.ilk")).unwrap();
        let env = resolve(&ilk, Path::new("test.ilk")).unwrap();
        let kli = parse_kli(kli_src, Path::new("test.kli")).unwrap();
        let ctx = ValidationContext::new(&env, &kli, Path::new("test.kli"));
        let mut errors = Vec::new();
        validate_structural(&ctx, &kli, &mut errors);
        validate_source(&ctx, &kli, &mut errors);
        errors
    }

    #[test]
    fn test_generated_exempt() {
        let errors = validate_source_pair(
            "Event {timestamp Int}\n@main\nCmd {\n  fields {...}\n  @source [fields]\n  emits []Event\n}",
            "e = Event {timestamp Int}\ncmd = Cmd {\n  fields {x Int}\n  emits [e & {timestamp Int*}]\n}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_mapped_valid() {
        let errors = validate_source_pair(
            "Event {id String}\n@main\nCmd {\n  fields {...}\n  @source [fields]\n  emits []Event\n}",
            "e = Event {id String}\ncmd = Cmd {\n  fields {userId String}\n  emits [e & {id String = fields.userId}]\n}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_source_not_found() {
        let errors = validate_source_pair(
            "Event {id String}\n@main\nCmd {\n  fields {...}\n  @source [fields]\n  emits []Event\n}",
            "e = Event {id String}\ncmd = Cmd {\n  fields {x Int}\n  emits [e & {id String = other.id}]\n}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_concrete_exempt() {
        let errors = validate_source_pair(
            "Event {status Concrete<Int>}\n@main\nCmd {\n  fields {...}\n  @source [fields]\n  emits []Event\n}",
            "e = Event {status 200}\ncmd = Cmd {\n  fields {x Int}\n  emits [e & {status 200}]\n}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }
}
