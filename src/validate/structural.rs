use crate::error::Diagnostic;
use crate::ilk::ast::*;
use crate::ilk::TypeEnv;
use crate::kli::ast::*;
use crate::span::{Span, S};
use std::collections::HashMap;
use std::path::Path;

pub struct ValidationContext<'a> {
    pub env: &'a TypeEnv,
    pub bindings: HashMap<String, &'a S<Binding>>,
    pub path: &'a Path,
}

impl<'a> ValidationContext<'a> {
    pub fn new(env: &'a TypeEnv, kli: &'a KliFile, path: &'a Path) -> Self {
        let mut bindings = HashMap::new();
        for binding in &kli.bindings {
            bindings.insert(binding.node.name.node.clone(), binding);
        }
        Self {
            env,
            bindings,
            path,
        }
    }
}

pub fn validate_structural(
    ctx: &ValidationContext,
    kli: &KliFile,
    errors: &mut Vec<Diagnostic>,
) {
    let main = match ctx.env.main() {
        Some(m) => m,
        None => {
            errors.push(Diagnostic::error(0..0, "No @main block in schema", ctx.path));
            return;
        }
    };

    // The main block should be a struct that contains all the bindings
    validate_main_against_bindings(ctx, main, kli, errors);
}

fn validate_main_against_bindings(
    ctx: &ValidationContext,
    _main: &S<Block>,
    kli: &KliFile,
    errors: &mut Vec<Diagnostic>,
) {
    // Find the main binding that matches @main type
    for binding in &kli.bindings {
        let type_name = &binding.node.type_name.node;

        // Check if the type exists
        if let Some(block) = ctx.env.get(type_name) {
            validate_value_against_type(ctx, &binding.node.body, &block.node.body, errors);

            // Validate associations
            validate_associations(ctx, binding, block, errors);
        } else {
            // Check if it's a base type (shouldn't happen for bindings typically)
            if !is_base_type(type_name) {
                errors.push(Diagnostic::error(
                    binding.node.type_name.span.clone(),
                    format!("Unknown type: {}", type_name),
                    ctx.path,
                ));
            }
        }
    }
}

fn validate_associations(
    ctx: &ValidationContext,
    binding: &S<Binding>,
    block: &S<Block>,
    errors: &mut Vec<Diagnostic>,
) {
    // Find @assoc annotation
    let assoc_types: Vec<&String> = block
        .node
        .annotations
        .iter()
        .filter_map(|a| match &a.node {
            Annotation::Assoc(types) => Some(types.iter().map(|t| &t.node).collect::<Vec<_>>()),
            _ => None,
        })
        .flatten()
        .collect();

    for assoc in &binding.node.assocs {
        // Check that the referenced binding exists
        if let Some(assoc_binding) = ctx.bindings.get(&assoc.node) {
            // Check that the binding's type is in the @assoc list
            let assoc_type = &assoc_binding.node.type_name.node;
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
                format!("Unknown binding in association: {}", assoc.node),
                ctx.path,
            ));
        }
    }
}

fn type_matches_assoc(assoc_type: &str, binding_type: &str, env: &TypeEnv) -> bool {
    if assoc_type == binding_type {
        return true;
    }

    // Check if binding_type is a variant of assoc_type (union)
    if let Some(block) = env.get(assoc_type) {
        if let TypeExpr::Union(variants) = &block.node.body.node {
            for variant in variants {
                if let TypeExpr::Named(name) = &variant.node {
                    if name == binding_type {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn validate_value_against_type(
    ctx: &ValidationContext,
    value: &S<KliValue>,
    ty: &S<TypeExpr>,
    errors: &mut Vec<Diagnostic>,
) {
    match (&value.node, &ty.node) {
        // Type references
        (KliValue::TypeRef(kli_type), TypeExpr::Base(base)) => {
            if !type_ref_matches_base(kli_type, base) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Type mismatch: expected {:?}, got {}", base, kli_type),
                    ctx.path,
                ));
            }
        }

        // Open type - kli must use same type ref
        (KliValue::TypeRef(_), TypeExpr::Named(_)) => {
            // TypeRef against named type - this is valid for open types
        }

        // Concrete type - kli must provide a literal
        (KliValue::LitString(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::String)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    "String literal doesn't match Concrete type",
                    ctx.path,
                ));
            }
        }
        (KliValue::LitInt(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::Int)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    "Int literal doesn't match Concrete type",
                    ctx.path,
                ));
            }
        }
        (KliValue::LitBool(_), TypeExpr::Concrete(inner)) => {
            if !matches!(&inner.node, TypeExpr::Base(BaseType::Bool)) {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    "Bool literal doesn't match Concrete type",
                    ctx.path,
                ));
            }
        }

        // Schema-fixed literals - must match exactly
        (KliValue::LitString(s), TypeExpr::LitString(expected)) => {
            if s != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected \"{}\", got \"{}\"", expected, s),
                    ctx.path,
                ));
            }
        }
        (KliValue::LitInt(n), TypeExpr::LitInt(expected)) => {
            if n != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected {}, got {}", expected, n),
                    ctx.path,
                ));
            }
        }
        (KliValue::LitBool(b), TypeExpr::LitBool(expected)) => {
            if b != expected {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Literal mismatch: expected {}, got {}", expected, b),
                    ctx.path,
                ));
            }
        }

        // Literal against open type - error
        (KliValue::LitString(_), TypeExpr::Base(BaseType::String)) => {
            errors.push(Diagnostic::error(
                value.span.clone(),
                "Cannot use literal for open String type - use String type ref",
                ctx.path,
            ));
        }

        // Structs
        (KliValue::Struct(kli_fields), TypeExpr::Struct(kind)) => {
            validate_struct(ctx, kli_fields, kind, &value.span, errors);
        }

        // Lists
        (KliValue::List(elements), TypeExpr::List(card, elem_ty)) => {
            validate_list(ctx, elements, card, elem_ty, &value.span, errors);
        }

        // References
        (KliValue::BindingRef(name), TypeExpr::Reference(expected_type)) => {
            if let Some(binding) = ctx.bindings.get(name) {
                let binding_type = &binding.node.type_name.node;
                if !type_matches_ref(binding_type, expected_type, ctx.env) {
                    errors.push(Diagnostic::error(
                        value.span.clone(),
                        format!(
                            "Reference type mismatch: {} is type {}, expected {}",
                            name, binding_type, expected_type
                        ),
                        ctx.path,
                    ));
                }
            } else {
                errors.push(Diagnostic::error(
                    value.span.clone(),
                    format!("Unknown binding: {}", name),
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
            // Check if left is open struct - if so, only validate required fields from right
            if matches!(&left.node, TypeExpr::Struct(StructKind::Open(_))) {
                // Open struct on left - extra fields allowed
                // Just check that required fields from right are present and valid
                if let (KliValue::Struct(kli_fields), TypeExpr::Struct(StructKind::Closed(ilk_fields))) =
                    (&value.node, &right.node)
                {
                    for ilk_field in ilk_fields {
                        let name = &ilk_field.node.name.node;
                        if let Some(kli_field) =
                            kli_fields.iter().find(|f| &f.node.name.node == name)
                        {
                            validate_value_against_type(
                                ctx,
                                &kli_field.node.value,
                                &ilk_field.node.ty,
                                errors,
                            );
                        }
                        // If field not present, it's optional by default in ilk
                    }
                } else {
                    // Fallback to checking both sides
                    validate_value_against_type(ctx, value, left, errors);
                    validate_value_against_type(ctx, value, right, errors);
                }
            } else {
                // Both sides have strict requirements
                validate_value_against_type(ctx, value, left, errors);
                validate_value_against_type(ctx, value, right, errors);
            }
        }

        // Named types - resolve and validate
        (_, TypeExpr::Named(name)) => {
            if let Some(block) = ctx.env.get(name) {
                validate_value_against_type(ctx, value, &block.node.body, errors);
            }
        }

        // Variant (kli) against block (ilk)
        (KliValue::Variant(variant_name, body), _) => {
            if let Some(block) = ctx.env.get(variant_name) {
                validate_value_against_type(ctx, body, &block.node.body, errors);
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
    kli_fields: &[S<KliField>],
    kind: &StructKind,
    span: &Span,
    errors: &mut Vec<Diagnostic>,
) {
    match kind {
        StructKind::Closed(ilk_fields) => {
            // Check all kli fields are in ilk
            for kli_field in kli_fields {
                let name = &kli_field.node.name.node;
                if let Some(ilk_field) = ilk_fields.iter().find(|f| &f.node.name.node == name) {
                    validate_value_against_type(
                        ctx,
                        &kli_field.node.value,
                        &ilk_field.node.ty,
                        errors,
                    );
                } else {
                    errors.push(Diagnostic::error(
                        kli_field.node.name.span.clone(),
                        format!("Extra field not in schema: {}", name),
                        ctx.path,
                    ));
                }
            }
        }

        StructKind::Open(ilk_fields) => {
            // Check required fields are present and match
            for ilk_field in ilk_fields {
                let name = &ilk_field.node.name.node;
                if let Some(kli_field) = kli_fields.iter().find(|f| &f.node.name.node == name) {
                    validate_value_against_type(
                        ctx,
                        &kli_field.node.value,
                        &ilk_field.node.ty,
                        errors,
                    );
                }
                // Open structs don't require fields
            }
        }

        StructKind::Anonymous(types) => {
            // Check cardinality
            if kli_fields.len() != types.len() {
                errors.push(Diagnostic::error(
                    span.clone(),
                    format!(
                        "Wrong field count: expected {}, got {}",
                        types.len(),
                        kli_fields.len()
                    ),
                    ctx.path,
                ));
                return;
            }

            // Check field types if specified
            for (kli_field, ty) in kli_fields.iter().zip(types.iter()) {
                if let Some(expected_ty) = ty {
                    validate_value_against_type(ctx, &kli_field.node.value, expected_ty, errors);
                }
            }
        }
    }
}

fn validate_list(
    ctx: &ValidationContext,
    elements: &[S<KliListElement>],
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
            KliListElement::Value(v) => {
                let spanned = S::new(v.clone(), elem.span.clone());
                validate_value_against_type(ctx, &spanned, elem_ty, errors);
            }
            KliListElement::BindingRef(name) => {
                if let Some(binding) = ctx.bindings.get(name) {
                    let binding_type = &binding.node.type_name.node;
                    // Check binding type matches list element type
                    if let TypeExpr::Named(expected) = &elem_ty.node {
                        if !type_matches_ref(binding_type, expected, ctx.env) {
                            errors.push(Diagnostic::error(
                                elem.span.clone(),
                                format!(
                                    "List element type mismatch: {} is {}, expected {}",
                                    name, binding_type, expected
                                ),
                                ctx.path,
                            ));
                        }
                    }
                } else {
                    errors.push(Diagnostic::error(
                        elem.span.clone(),
                        format!("Unknown binding in list: {}", name),
                        ctx.path,
                    ));
                }
            }
            KliListElement::Refinement(name, _fields) => {
                // Refinements are handled in @source validation
                if !ctx.bindings.contains_key(name) {
                    errors.push(Diagnostic::error(
                        elem.span.clone(),
                        format!("Unknown binding in refinement: {}", name),
                        ctx.path,
                    ));
                }
            }
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

fn type_matches_ref(binding_type: &str, expected_type: &str, env: &TypeEnv) -> bool {
    if binding_type == expected_type {
        return true;
    }

    // Check if binding_type is a variant of expected_type (union)
    if let Some(block) = env.get(expected_type) {
        if let TypeExpr::Union(variants) = &block.node.body.node {
            for variant in variants {
                if let TypeExpr::Named(name) = &variant.node {
                    if name == binding_type {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn is_base_type(name: &str) -> bool {
    matches!(
        name,
        "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ilk::{parse_ilk, resolve};
    use crate::kli::parse_kli;
    use std::path::Path;

    fn validate_pair(ilk_src: &str, kli_src: &str) -> Vec<Diagnostic> {
        let ilk = parse_ilk(ilk_src, Path::new("test.ilk")).unwrap();
        let env = resolve(&ilk, Path::new("test.ilk")).unwrap();
        let kli = parse_kli(kli_src, Path::new("test.kli")).unwrap();
        let ctx = ValidationContext::new(&env, &kli, Path::new("test.kli"));
        let mut errors = Vec::new();
        validate_structural(&ctx, &kli, &mut errors);
        errors
    }

    #[test]
    fn test_type_match() {
        let errors = validate_pair(
            "@main\nFoo {x String}",
            "foo = Foo {x String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_type_mismatch() {
        let errors = validate_pair(
            "@main\nFoo {x Int}",
            "foo = Foo {x String}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_concrete_type() {
        let errors = validate_pair(
            "@main\nFoo {x Concrete<String>}",
            "foo = Foo {x \"hello\"}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_literal_vs_open() {
        let errors = validate_pair(
            "@main\nFoo {x String}",
            "foo = Foo {x \"hello\"}",
        );
        assert!(!errors.is_empty()); // literal can't satisfy open type
    }

    #[test]
    fn test_schema_fixed_literal() {
        let errors = validate_pair(
            "@main\nFoo {x \"hello\"}",
            "foo = Foo {x \"hello\"}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_literal_mismatch() {
        let errors = validate_pair(
            "@main\nFoo {x \"hello\"}",
            "foo = Foo {x \"world\"}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_extra_field() {
        let errors = validate_pair(
            "@main\nFoo {x Int}",
            "foo = Foo {x Int, y Int}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_open_struct() {
        let errors = validate_pair(
            "@main\nFoo {...}",
            "foo = Foo {x Int, y String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_anonymous_struct() {
        let errors = validate_pair(
            "@main\nFoo {_}",
            "foo = Foo {a String}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_anonymous_struct_wrong_count() {
        let errors = validate_pair(
            "@main\nFoo {_}",
            "foo = Foo {a Int, b Int}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_list_any() {
        let errors = validate_pair(
            "Item {x Int}\n@main\nFoo {items []Item}",
            "i1 = Item {x Int}\ni2 = Item {x Int}\nfoo = Foo {items [i1, i2]}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_list_cardinality() {
        let errors = validate_pair(
            "Item {x Int}\n@main\nFoo {items [3]Item}",
            "i1 = Item {x Int}\ni2 = Item {x Int}\nfoo = Foo {items [i1, i2]}",
        );
        assert!(!errors.is_empty()); // expected 3, got 2
    }
}
