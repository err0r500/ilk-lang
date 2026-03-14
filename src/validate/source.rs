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
    // Get the struct fields from the type body
    let fields = match &type_decl.body.node {
        TypeExpr::Struct(StructKind::Closed(f) | StructKind::Open(f)) => f,
        TypeExpr::Intersection(left, right) => {
            let mut all_fields = Vec::new();
            collect_struct_fields(&left.node, &mut all_fields);
            collect_struct_fields(&right.node, &mut all_fields);
            validate_intersection_sources(ctx, inst, &all_fields, errors);
            return;
        }
        _ => return,
    };

    // Check each field for @source annotation
    for type_field in fields {
        let source_ann = type_field
            .node
            .annotations
            .iter()
            .find_map(|a| match &a.node {
                Annotation::Source(paths) => Some(paths),
                _ => None,
            });

        if let Some(sources) = source_ann {
            if let Value::Struct(inst_fields) = &inst.body.node {
                if let Some(inst_field) = inst_fields
                    .iter()
                    .find(|f| f.node.name.node == type_field.node.name.node)
                {
                    validate_field_source(ctx, inst_field, sources, inst, errors);
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
    inst: &Instance,
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
            if let Value::Struct(inst_fields) = &inst.body.node {
                if let Some(inst_field) = inst_fields
                    .iter()
                    .find(|f| f.node.name.node == type_field.node.name.node)
                {
                    validate_field_source(ctx, inst_field, sources, inst, errors);
                }
            }
        }
    }
}

fn validate_field_source(
    ctx: &ValidationContext,
    inst_field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    inst: &Instance,
    errors: &mut Vec<Diagnostic>,
) {
    // Check if field is a list with refinements
    if let Value::List(elements) = &inst_field.node.value.node {
        for elem in elements {
            match &elem.node {
                ListElement::Refinement(_name, ref_fields) => {
                    for ref_field in ref_fields {
                        validate_refinement_field(ctx, ref_field, sources, inst, errors);
                    }
                }
                ListElement::BindingRef(name) => {
                    if let Some(ref_inst) = ctx.get_instance(name) {
                        if let Value::Struct(ref_fields) = &ref_inst.body.node {
                            for ref_field in ref_fields {
                                validate_refinement_field(ctx, ref_field, sources, inst, errors);
                            }
                        }
                    }
                }
                ListElement::Value(v) => match v {
                    Value::Struct(fields) => {
                        for field in fields {
                            validate_refinement_field(ctx, field, sources, inst, errors);
                        }
                    }
                    Value::BindingRef(name) => {
                        if let Some(ref_inst) = ctx.get_instance(name) {
                            if let Value::Struct(ref_fields) = &ref_inst.body.node {
                                for ref_field in ref_fields {
                                    validate_refinement_field(
                                        ctx, ref_field, sources, inst, errors,
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
        for nested in nested_fields {
            validate_refinement_field(ctx, nested, sources, inst, errors);
        }
    }
}

fn validate_refinement_field(
    ctx: &ValidationContext,
    field: &S<InstanceField>,
    sources: &[S<SourcePath>],
    inst: &Instance,
    errors: &mut Vec<Diagnostic>,
) {
    let name = &field.node.name.node;

    match &field.node.origin {
        FieldOrigin::Generated => {
            // Exempt - no check needed
        }
        FieldOrigin::Mapped(path) => {
            if let Some(root) = path.first() {
                if !source_contains_root(sources, root) {
                    errors.push(Diagnostic::error(
                        field.span.clone(),
                        format!("Source path root '{}' not in @source list", root),
                        ctx.path,
                    ));
                } else {
                    validate_source_path(ctx, path, field, inst, errors);
                }
            }
        }
        FieldOrigin::Computed(paths) => {
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
            let found = check_implicit_source(ctx, name, sources, inst);
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

fn source_contains_root(sources: &[S<SourcePath>], root: &str) -> bool {
    sources.iter().any(|s| match &s.node {
        SourcePath::Simple(name) => name == root,
        SourcePath::Dotted(parts) => parts.first().map(|p| p == root).unwrap_or(false),
    })
}

fn validate_source_path(
    ctx: &ValidationContext,
    path: &[String],
    field: &S<InstanceField>,
    inst: &Instance,
    errors: &mut Vec<Diagnostic>,
) {
    if let Value::Struct(inst_fields) = &inst.body.node {
        let mut current_fields = inst_fields.as_slice();
        let mut found = true;

        for (i, segment) in path.iter().enumerate() {
            if let Some(f) = current_fields.iter().find(|f| &f.node.name.node == segment) {
                if i < path.len() - 1 {
                    if let Value::Struct(nested) = &f.node.value.node {
                        current_fields = nested;
                    } else {
                        found = false;
                        break;
                    }
                }
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
    inst: &Instance,
) -> bool {
    if let Value::Struct(inst_fields) = &inst.body.node {
        for source in sources {
            let root = match &source.node {
                SourcePath::Simple(name) => name,
                SourcePath::Dotted(parts) => parts.first().unwrap(),
            };

            if let Some(source_field) = inst_fields.iter().find(|f| &f.node.name.node == root) {
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
}
