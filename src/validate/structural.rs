use crate::ast::*;
use crate::error::Diagnostic;
use crate::resolve::TypeEnv;
use crate::span::{Span, S};
use std::path::Path;

fn format_type(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Base(b) => format!("{:?}", b),
        TypeExpr::Named(n) => n.clone(),
        TypeExpr::Concrete(inner) => format!("Concrete<{}>", format_type(&inner.node)),
        TypeExpr::Reference(n) => format!("&{}", n),
        TypeExpr::List(_, inner) => format!("[]{}", format_type(&inner.node)),
        TypeExpr::LitString(s) => format!("\"{}\"", s),
        TypeExpr::LitInt(i) => format!("{}", i),
        TypeExpr::LitBool(b) => format!("{}", b),
        _ => "?".to_string(),
    }
}

pub struct ValidationContext<'a> {
    pub env: &'a TypeEnv,
    pub path: &'a Path,
}

impl<'a> ValidationContext<'a> {
    pub fn new(env: &'a TypeEnv, path: &'a Path) -> Self {
        Self { env, path }
    }

    pub fn get_instance(&self, name: &str) -> Option<&Instance> {
        self.env.get_instance(name).map(|s| &s.node)
    }
}

pub fn validate_structural(
    ctx: &ValidationContext,
    file: &File,
    errors: &mut Vec<Diagnostic>,
) {
    // Validate each instance against its type
    for inst in file.instances() {
        let type_name = &inst.type_name.node;

        if let Some(type_decl) = ctx.env.get_type(type_name) {
            validate_value_against_type(ctx, &inst.body, &type_decl.node.body, errors);

            // Validate associations
            validate_associations(ctx, inst, &type_decl.node, errors);
        }
        // Unknown type errors are already caught in resolve
    }
}

fn validate_associations(
    ctx: &ValidationContext,
    inst: &Instance,
    type_decl: &TypeDecl,
    errors: &mut Vec<Diagnostic>,
) {
    // Find @assoc annotation
    let assoc_types: Vec<&String> = type_decl
        .annotations
        .iter()
        .filter_map(|a| match &a.node {
            Annotation::Assoc(types) => Some(types.iter().map(|t| &t.node).collect::<Vec<_>>()),
            _ => None,
        })
        .flatten()
        .collect();

    for assoc in &inst.assocs {
        // Check that the referenced instance exists
        if let Some(assoc_inst) = ctx.get_instance(&assoc.node) {
            // Check that the instance's type is in the @assoc list
            let assoc_type = &assoc_inst.type_name.node;
            if !assoc_types.iter().any(|t| type_matches_assoc(*t, assoc_type, ctx.env)) {
                errors.push(Diagnostic::error(
                    assoc.span.clone(),
                    format!(
                        "Association {} (type {}) not allowed - expected one of {:?}",
                        assoc.node, assoc_type, assoc_types
                    ),
                    ctx.path,
                ));
            }
        } else {
            errors.push(Diagnostic::error(
                assoc.span.clone(),
                format!("Unknown instance in association: {}", assoc.node),
                ctx.path,
            ));
        }
    }
}

fn type_matches_assoc(assoc_type: &str, instance_type: &str, env: &TypeEnv) -> bool {
    type_matches_ref(instance_type, assoc_type, env)
}

fn validate_value_against_type(
    ctx: &ValidationContext,
    value: &S<Value>,
    ty: &S<TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    match (&value.node, &ty.node) {
        // Type references
        (Value::TypeRef(val_type), TypeExpr::Base(base)) => {
            if !type_ref_matches_base(val_type, base) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Type mismatch: expected {:?}, got {}", base, val_type),
                    ctx.path,
                ));
            }
        }

        // Open type - value must use same type ref
        (Value::TypeRef(_), TypeExpr::Named(_)) => {
            // TypeRef against named type - valid for open types
        }

        // Concrete type - must provide a literal
        (Value::LitString(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::String)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("String literal doesn't match expected type {}", format_type(&ty.node)),
                    ctx.path,
                ));
            }
        }
        (Value::LitInt(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::Int)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Int literal doesn't match expected type {}", format_type(&ty.node)),
                    ctx.path,
                ));
            }
        }
        (Value::LitBool(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::Bool)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Bool literal doesn't match expected type {}", format_type(&ty.node)),
                    ctx.path,
                ));
            }
        }

        // Schema-fixed literals - must match exactly
        (Value::LitString(s), TypeExpr::LitString(expected)) => {
            if s != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected \"{}\", got \"{}\"", expected, s),
                    ctx.path,
                ));
            }
        }
        (Value::LitInt(n), TypeExpr::LitInt(expected)) => {
            if n != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected {}, got {}", expected, n),
                    ctx.path,
                ));
            }
        }
        (Value::LitBool(b), TypeExpr::LitBool(expected)) => {
            if b != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected {}, got {}", expected, b),
                    ctx.path,
                ));
            }
        }

        // Literal against open type - error
        (Value::LitString(_), TypeExpr::Base(BaseType::String)) => {
            errors.push(Diagnostic::error(
                value.span.clone(),
                "Cannot use literal for open String type - use String type ref",
                ctx.path,
            ));
        }

        // Structs
        (Value::Struct(val_fields), TypeExpr::Struct(kind)) => {
            validate_struct(ctx, val_fields, kind, &value.span, errors);
        }

        // Lists
        (Value::List(elements), TypeExpr::List(card, elem_ty)) => {
            validate_list(ctx, elements, card, elem_ty, &value.span, errors);
        }

        // References
        (Value::BindingRef(name), TypeExpr::Reference(expected_type)) => {
            if let Some(inst) = ctx.get_instance(name) {
                let inst_type = &inst.type_name.node;
                if !type_matches_ref(inst_type, expected_type, ctx.env) {
                    errors.push(Diagnostic::error(
                        value.span.clone(),
                        format!(
                            "Reference type mismatch: {} is type {}, expected {}",
                            name, inst_type, expected_type
                        ),
                        ctx.path,
                    ));
                }
            } else {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Unknown instance: {}", name),
                    ctx.path,
                ));
            }
        }

        // Unions
        (_, TypeExpr::Union(variants)) => {
            let mut matched = false;
            for variant in variants {
                let mut variant_errors = Vec::new();
                validate_value_against_type(ctx, value, variant, &mut variant_errors);
                if variant_errors.is_empty() {
                    matched = true;
                    break;
                }
            }
            if !matched {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    "Value doesn't match any union variant",
                    ctx.path,
                ));
            }
        }

        // Intersections
        (_, TypeExpr::Intersection(left, right)) => {
            if matches!(&left.node, TypeExpr::Struct(StructKind::Open(_))) {
                // Open struct on left - extra fields allowed
                if let (Value::Struct(val_fields), TypeExpr::Struct(StructKind::Closed(type_fields))) =
                    (&value.node, &right.node)
                {
                    for type_field in type_fields {
                        let name = &type_field.node.name.node;
                        if let Some(val_field) =
                            val_fields.iter().find(|f| &f.node.name.node == name)
                        {
                            validate_value_against_type(
                                ctx,
                                &val_field.node.value,
                                &type_field.node.ty,
                                errors,
                            );
                        }
                    }
                } else {
                    validate_value_against_type(ctx, value, left, errors);
                    validate_value_against_type(ctx, value, right, errors);
                }
            } else {
                validate_value_against_type(ctx, value, left, errors);
                validate_value_against_type(ctx, value, right, errors);
            }
        }

        // Named types - resolve and validate
        (_, TypeExpr::Named(name)) => {
            if let Some(type_decl) = ctx.env.get_type(name) {
                validate_value_against_type(ctx, value, &type_decl.node.body, errors);
            }
        }

        // RefinableRef - same as Named but allows concrete refinements
        (_, TypeExpr::RefinableRef(name)) => {
            if let Some(type_decl) = ctx.env.get_type(name) {
                validate_value_against_type(ctx, value, &type_decl.node.body, errors);
            }
        }

        // Variant value against type
        (Value::Variant(variant_name, body), _) => {
            if let Some(type_decl) = ctx.env.get_type(variant_name) {
                validate_value_against_type(ctx, body, &type_decl.node.body, errors);
            } else {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Unknown variant type: {}", variant_name),
                    ctx.path,
                ));
            }
        }

        // Wildcard accepts anything
        (_, TypeExpr::Base(BaseType::Wildcard)) => {}

        _ => {
            // Other mismatches
        }
    }
}

fn validate_struct(
    ctx: &ValidationContext,
    val_fields: &[S<InstanceField>],
    kind: &StructKind,
    span: &Span,
    errors: &mut Vec<Diagnostic>,
) {
    match kind {
        StructKind::Closed(type_fields) => {
            // Check all value fields are in type
            for val_field in val_fields {
                let name = &val_field.node.name.node;
                if let Some(type_field) = type_fields.iter().find(|f| &f.node.name.node == name) {
                    validate_value_against_type(
                        ctx,
                        &val_field.node.value,
                        &type_field.node.ty,
                        errors,
                    );
                } else {
                    errors.push(Diagnostic::error(
                        val_field.node.name.span.clone(),
                        format!("Extra field not in schema: {}", name),
                        ctx.path,
                    ));
                }
            }

            // Check all required type fields are present
            for type_field in type_fields {
                if type_field.node.optional {
                    continue;
                }
                let name = &type_field.node.name.node;
                if !val_fields.iter().any(|f| &f.node.name.node == name) {
                    errors.push(Diagnostic::error(
                        span.clone(),
                        format!("Missing required field: {}", name),
                        ctx.path,
                    ));
                }
            }
        }

        StructKind::Open(type_fields) => {
            // Check required fields are present and match
            for type_field in type_fields {
                let name = &type_field.node.name.node;
                if let Some(val_field) = val_fields.iter().find(|f| &f.node.name.node == name) {
                    validate_value_against_type(
                        ctx,
                        &val_field.node.value,
                        &type_field.node.ty,
                        errors,
                    );
                }
            }
        }

        StructKind::Anonymous(types) => {
            // Check cardinality
            if val_fields.len() != types.len() {
                errors.push(Diagnostic::error(
                    span.clone(),
                    format!(
                        "Wrong field count: expected {}, got {}",
                        types.len(),
                        val_fields.len()
                    ),
                    ctx.path,
                ));
                return;
            }

            // Check field types if specified
            for (val_field, ty) in val_fields.iter().zip(types.iter()) {
                if let Some(expected_ty) = ty {
                    validate_value_against_type(ctx, &val_field.node.value, expected_ty, errors);
                }
            }
        }
    }
}

fn validate_list(
    ctx: &ValidationContext,
    elements: &[S<ListElement>],
    card: &Cardinality,
    elem_ty: &S<TypeExpr>,
    span: &Span,
    errors: &mut Vec<Diagnostic>,
) {
    let len = elements.len();

    // Check cardinality
    let valid_card = match card {
        Cardinality::Any => true,
        Cardinality::Exact(n) => len == *n,
        Cardinality::AtLeast(n) => len >= *n,
        Cardinality::AtMost(n) => len <= *n,
        Cardinality::Range(min, max) => len >= *min && len <= *max,
    };

    if !valid_card {
        errors.push(Diagnostic::error(
            span.clone(),
            format!("List cardinality mismatch: got {} elements, expected {:?}", len, card),
            ctx.path,
        ));
    }

    // Check each element
    for elem in elements {
        match &elem.node {
            ListElement::Value(v) => {
                let spanned = S::new(v.clone(), elem.span.clone());
                validate_value_against_type(ctx, &spanned, elem_ty, errors);
            }
            ListElement::BindingRef(name) => {
                if let Some(inst) = ctx.get_instance(name) {
                    let inst_type = &inst.type_name.node;
                    let expected = match &elem_ty.node {
                        TypeExpr::Named(t) => Some(t),
                        TypeExpr::RefinableRef(t) => Some(t),
                        _ => None,
                    };
                    if let Some(expected) = expected {
                        if !type_matches_ref(inst_type, expected, ctx.env) {
                            errors.push(Diagnostic::error(
                                elem.span.clone(),
                                format!(
                                    "List element type mismatch: {} is {}, expected {}",
                                    name, inst_type, expected
                                ),
                                ctx.path,
                            ));
                        }
                    }
                } else {
                    errors.push(Diagnostic::error(
                        elem.span.clone(),
                        format!("Unknown instance in list: {}", name),
                        ctx.path,
                    ));
                }
            }
            ListElement::Refinement(name, fields) => {
                if let Some(inst) = ctx.get_instance(name) {
                    let inst_type = &inst.type_name.node;

                    // Check type matches (for RefinableRef, extract the inner type name)
                    let expected_type = match &elem_ty.node {
                        TypeExpr::RefinableRef(t) => t,
                        TypeExpr::Named(t) => t,
                        _ => {
                            errors.push(Diagnostic::error(
                                elem.span.clone(),
                                "Refinement on non-named type",
                                ctx.path,
                            ));
                            continue;
                        }
                    };

                    if !type_matches_ref(inst_type, expected_type, ctx.env) {
                        errors.push(Diagnostic::error(
                            elem.span.clone(),
                            format!(
                                "Refinement type mismatch: {} is {}, expected {}",
                                name, inst_type, expected_type
                            ),
                            ctx.path,
                        ));
                    }

                    // Validate refinement fields against the instance's actual fields
                    let is_refinable = matches!(&elem_ty.node, TypeExpr::RefinableRef(_));
                    validate_refinement_fields_against_instance(ctx, fields, inst, is_refinable, errors);
                } else {
                    errors.push(Diagnostic::error(
                        elem.span.clone(),
                        format!("Unknown instance in refinement: {}", name),
                        ctx.path,
                    ));
                }
            }
        }
    }
}

/// Extract field type from a TypeExpr (handles Named, Intersection, Struct)
fn get_field_type_from_type_expr<'a>(
    ctx: &'a ValidationContext,
    ty: &'a TypeExpr,
    field_name: &str,
) -> Option<&'a S<TypeExpr>> {
    match ty {
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            fields
                .iter()
                .find(|f| f.node.name.node == field_name)
                .map(|f| &f.node.ty)
        }
        TypeExpr::Named(name) | TypeExpr::RefinableRef(name) => {
            ctx.env
                .get_type(name)
                .and_then(|decl| get_field_type_from_type_expr(ctx, &decl.node.body.node, field_name))
        }
        TypeExpr::Intersection(left, right) => {
            get_field_type_from_type_expr(ctx, &left.node, field_name)
                .or_else(|| get_field_type_from_type_expr(ctx, &right.node, field_name))
        }
        _ => None,
    }
}

/// Check if a refinement value is compatible with a declared field type
fn refinement_value_matches_type(
    ctx: &ValidationContext,
    value: &Value,
    declared_type: &TypeExpr,
) -> bool {
    match (value, declared_type) {
        // Literals against base types
        (Value::LitString(_), TypeExpr::Base(BaseType::String)) => true,
        (Value::LitString(_), TypeExpr::Concrete(inner)) => {
            matches!(&inner.node, TypeExpr::Base(BaseType::String))
        }
        (Value::LitInt(_), TypeExpr::Base(BaseType::Int)) => true,
        (Value::LitInt(_), TypeExpr::Concrete(inner)) => {
            matches!(&inner.node, TypeExpr::Base(BaseType::Int))
        }
        (Value::LitBool(_), TypeExpr::Base(BaseType::Bool)) => true,
        (Value::LitBool(_), TypeExpr::Concrete(inner)) => {
            matches!(&inner.node, TypeExpr::Base(BaseType::Bool))
        }
        // Literal against literal type - must match exactly
        (Value::LitString(s), TypeExpr::LitString(expected)) => s == expected,
        (Value::LitInt(n), TypeExpr::LitInt(expected)) => n == expected,
        (Value::LitBool(b), TypeExpr::LitBool(expected)) => b == expected,
        // TypeRef against base type
        (Value::TypeRef(name), TypeExpr::Base(base)) => type_ref_matches_base(name, base),
        // TypeRef against named type
        (Value::TypeRef(_), TypeExpr::Named(_) | TypeExpr::RefinableRef(_)) => true,
        // Resolve named types
        (_, TypeExpr::Named(name) | TypeExpr::RefinableRef(name)) => {
            // First try to resolve as base type
            let base = match name.as_str() {
                "String" => Some(BaseType::String),
                "Int" => Some(BaseType::Int),
                "Bool" => Some(BaseType::Bool),
                "Float" => Some(BaseType::Float),
                "Uuid" => Some(BaseType::Uuid),
                "Date" => Some(BaseType::Date),
                "Timestamp" => Some(BaseType::Timestamp),
                "Money" => Some(BaseType::Money),
                _ => None,
            };
            if let Some(base) = base {
                refinement_value_matches_type(ctx, value, &TypeExpr::Base(base))
            } else if let Some(decl) = ctx.env.get_type(name) {
                refinement_value_matches_type(ctx, value, &decl.node.body.node)
            } else {
                false
            }
        }
        // Union - any variant must match
        (_, TypeExpr::Union(variants)) => {
            variants.iter().any(|v| refinement_value_matches_type(ctx, value, &v.node))
        }
        // String literals are valid for Uuid, Date, Timestamp (they're string-based types)
        (Value::LitString(_), TypeExpr::Base(BaseType::Uuid | BaseType::Date | BaseType::Timestamp)) => true,
        // Int literal doesn't match Uuid, Date, etc.
        (Value::LitInt(_), TypeExpr::Base(BaseType::Uuid | BaseType::Date | BaseType::Timestamp | BaseType::Float | BaseType::Money)) => false,
        (Value::LitString(_), TypeExpr::Base(BaseType::Int | BaseType::Float | BaseType::Bool | BaseType::Money)) => false,
        _ => false,
    }
}

fn validate_refinement_fields_against_instance(
    ctx: &ValidationContext,
    fields: &[S<InstanceField>],
    inst: &Instance,
    is_refinable: bool,
    errors: &mut Vec<Diagnostic>,
) {
    // Get fields from the instance's body
    let inst_fields = match &inst.body.node {
        Value::Struct(f) => f,
        _ => return,
    };

    // Get the type declaration for the instance
    let type_decl = ctx.env.get_type(&inst.type_name.node);

    for field in fields {
        let field_name = &field.node.name.node;
        if let Some(inst_field) = inst_fields.iter().find(|f| &f.node.name.node == field_name) {
            // Check if refinement value is a concrete literal
            let is_concrete = matches!(
                &field.node.value.node,
                Value::LitString(_) | Value::LitInt(_) | Value::LitBool(_)
            );

            // Check if the instance field expects an open type (TypeRef like String, Int)
            let is_open_type = matches!(&inst_field.node.value.node, Value::TypeRef(_));

            if is_concrete && is_open_type && !is_refinable {
                errors.push(Diagnostic::error(
                    field.span.clone(),
                    format!(
                        "Cannot use concrete value for open type field '{}' - type is not refinable (use -{})",
                        field_name, inst.type_name.node
                    ),
                    ctx.path,
                ));
            }

            // Validate refinement value type against declared field type
            // First try type declaration, then fall back to instance field's value type (for open structs)
            let declared_type: Option<TypeExpr> = type_decl
                .and_then(|td| get_field_type_from_type_expr(ctx, &td.node.body.node, field_name))
                .map(|t| t.node.clone())
                .or_else(|| {
                    // For open structs: infer type from instance field's value
                    match &inst_field.node.value.node {
                        Value::TypeRef(name) => Some(TypeExpr::Named(name.clone())),
                        _ => None,
                    }
                });

            if let Some(field_type) = declared_type {
                if !refinement_value_matches_type(ctx, &field.node.value.node, &field_type) {
                    errors.push(Diagnostic::error(
                        field.node.value.span.clone(),
                        format!(
                            "Type mismatch in refinement: field '{}' expects {}, got {}",
                            field_name,
                            format_type(&field_type),
                            match &field.node.value.node {
                                Value::LitInt(_) => "Int".to_string(),
                                Value::LitString(_) => "String".to_string(),
                                Value::LitBool(_) => "Bool".to_string(),
                                Value::TypeRef(t) => t.clone(),
                                _ => "?".to_string(),
                            }
                        ),
                        ctx.path,
                    ));
                }
            }
        } else {
            errors.push(Diagnostic::error(
                field.node.name.span.clone(),
                format!("Unknown field in refinement: {}", field_name),
                ctx.path,
            ));
        }
    }
}

fn type_ref_matches_base(type_ref: &str, base: &BaseType) -> bool {
    match (type_ref, base) {
        ("Uuid", BaseType::Uuid) => true,
        ("String", BaseType::String) => true,
        ("Int", BaseType::Int) => true,
        ("Float", BaseType::Float) => true,
        ("Bool", BaseType::Bool) => true,
        ("Date", BaseType::Date) => true,
        ("Timestamp", BaseType::Timestamp) => true,
        ("Money", BaseType::Money) => true,
        _ => false,
    }
}

fn type_matches_ref(inst_type: &str, expected_type: &str, env: &TypeEnv) -> bool {
    if inst_type == expected_type {
        return true;
    }

    // Check if inst_type is a variant of expected_type (union)
    if let Some(type_decl) = env.get_type(expected_type) {
        if let TypeExpr::Union(variants) = &type_decl.node.body.node {
            for variant in variants {
                if let TypeExpr::Named(name) = &variant.node {
                    if name == inst_type {
                        return true;
                    }
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::resolve::resolve;
    use std::path::Path;

    fn validate_src(src: &str) -> Vec<Diagnostic> {
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        validate_structural(&ctx, &file, &mut errors);
        errors
    }

    #[test]
    fn test_type_match() {
        let errors = validate_src(
            "type Foo = {x String}\nfoo = Foo {x String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_type_mismatch() {
        let errors = validate_src(
            "type Foo = {x Int}\nfoo = Foo {x String}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_concrete_type() {
        let errors = validate_src(
            "type Foo = {x Concrete<String>}\nfoo = Foo {x \"hello\"}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_literal_vs_open() {
        let errors = validate_src(
            "type Foo = {x String}\nfoo = Foo {x \"hello\"}",
        );
        assert!(!errors.is_empty()); // literal can't satisfy open type
    }

    #[test]
    fn test_schema_fixed_literal() {
        let errors = validate_src(
            "type Foo = {x \"hello\"}\nfoo = Foo {x \"hello\"}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_literal_mismatch() {
        let errors = validate_src(
            "type Foo = {x \"hello\"}\nfoo = Foo {x \"world\"}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_extra_field() {
        let errors = validate_src(
            "type Foo = {x Int}\nfoo = Foo {x Int, y Int}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_open_struct() {
        let errors = validate_src(
            "type Foo = {...}\nfoo = Foo {x Int, y String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_anonymous_struct() {
        let errors = validate_src(
            "type Foo = {_}\nfoo = Foo {a String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_anonymous_struct_wrong_count() {
        let errors = validate_src(
            "type Foo = {_}\nfoo = Foo {a Int, b Int}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_list_any() {
        let errors = validate_src(
            "type Item = {x Int}\ntype Foo = {items []Item}\ni1 = Item {x Int}\ni2 = Item {x Int}\nfoo = Foo {items [i1, i2]}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_list_cardinality() {
        let errors = validate_src(
            "type Item = {x Int}\ntype Foo = {items [3]Item}\ni1 = Item {x Int}\ni2 = Item {x Int}\nfoo = Foo {items [i1, i2]}",
        );
        assert!(!errors.is_empty()); // expected 3, got 2
    }

    #[test]
    fn test_refinable_ref_allows_concrete() {
        let errors = validate_src(r#"
type Event = {...} & {timestamp Int}
type Command = {
    emits []-Event
}
ev = Event {id String}
cmd = Command {
    emits [ev & {id "123"}]
}
"#);
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_non_refinable_rejects_concrete() {
        let errors = validate_src(r#"
type Event = {...} & {timestamp Int}
type Command = {
    emits []Event
}
ev = Event {id String}
cmd = Command {
    emits [ev & {id "123"}]
}
"#);
        assert!(!errors.is_empty()); // should error: concrete value on non-refinable
    }

    #[test]
    fn test_refinement_field_type_mismatch() {
        // id is Uuid, but refinement uses Int literal 123
        let errors = validate_src(r#"
type Event = {...} & {id Uuid}
type Scenario = {
    given []-Event
}
userRegistered = Event {id Uuid}
scenario = Scenario {
    given [userRegistered & {id 123}]
}
"#);
        assert!(!errors.is_empty(), "Should error: id is Uuid but got Int");
        assert!(errors.iter().any(|e| e.message.contains("Type mismatch")), "Expected type mismatch error: {:?}", errors);
    }

    #[test]
    fn test_refinement_field_type_matches() {
        // id is Concrete<String>, refinement uses String literal - should match
        let errors = validate_src(r#"
type Event = {...} & {id Concrete<String>}
type Scenario = {
    given []-Event
}
userRegistered = Event {id "placeholder"}
scenario = Scenario {
    given [userRegistered & {id "user-123"}]
}
"#);
        assert!(errors.is_empty(), "Should accept: id is Concrete<String> and refinement is String: {:?}", errors);
    }
}
