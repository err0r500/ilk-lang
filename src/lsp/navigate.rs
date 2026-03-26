use crate::ast::*;
use crate::resolve::TypeEnv;
use crate::span::{Span, S};

/// Find the definition span for the symbol at the given offset
pub fn find_definition(file: &File, env: &TypeEnv, offset: usize) -> Option<Span> {
    // Check meta declarations
    for item in &file.items {
        if let Item::MetaDecl(decl) = &item.node {
            if let Some(span) = find_in_type_expr(&decl.body, env, offset) {
                return Some(span);
            }
        }
    }

    // Check instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            // Instance type_name
            if inst.type_name.span.contains(&offset) {
                return env.get_meta(&inst.type_name.node).map(|t| t.span.clone());
            }
            // Instance body
            if let Some(span) = find_in_value(&inst.body, env, offset) {
                return Some(span);
            }
        }
    }

    None
}

fn find_in_type_expr(ty: &S<TypeExpr>, env: &TypeEnv, offset: usize) -> Option<Span> {
    if !ty.span.contains(&offset) {
        return None;
    }

    match &ty.node {
        TypeExpr::Named(name) | TypeExpr::Reference(name) | TypeExpr::RefinableRef(name) => {
            env.get_meta(name).map(|t| t.span.clone())
        }
        TypeExpr::Concrete(inner) | TypeExpr::List(_, inner) => {
            find_in_type_expr(inner, env, offset)
        }
        TypeExpr::Union(variants) => {
            for v in variants {
                if let Some(span) = find_in_type_expr(v, env, offset) {
                    return Some(span);
                }
            }
            None
        }
        TypeExpr::Intersection(left, right) => {
            find_in_type_expr(left, env, offset).or_else(|| find_in_type_expr(right, env, offset))
        }
        TypeExpr::Struct(kind) => {
            let fields = match kind {
                StructKind::Closed(f) | StructKind::Open(f) => f,
                StructKind::Anonymous(types) => {
                    for ty in types.iter().flatten() {
                        if let Some(span) = find_in_type_expr(ty, env, offset) {
                            return Some(span);
                        }
                    }
                    return None;
                }
            };
            for field in fields {
                if let Some(span) = find_in_type_expr(&field.node.ty, env, offset) {
                    return Some(span);
                }
            }
            None
        }
        _ => None,
    }
}

fn find_in_value(val: &S<Value>, env: &TypeEnv, offset: usize) -> Option<Span> {
    if !val.span.contains(&offset) {
        return None;
    }

    match &val.node {
        Value::TypeRef(name) => env.get_meta(name).map(|t| t.span.clone()),
        Value::BindingRef(name) => env.get_instance(name).map(|i| i.span.clone()),
        Value::Struct(fields) => {
            for field in fields {
                if let Some(span) = find_in_value(&field.node.value, env, offset) {
                    return Some(span);
                }
            }
            None
        }
        Value::List(elements) => {
            for elem in elements {
                match &elem.node {
                    ListElement::Value(v) => {
                        let v_spanned = S::new(v.clone(), elem.span.clone());
                        if let Some(span) = find_in_value(&v_spanned, env, offset) {
                            return Some(span);
                        }
                    }
                    ListElement::BindingRef(name) => {
                        if elem.span.contains(&offset) {
                            return env.get_instance(name).map(|i| i.span.clone());
                        }
                    }
                    ListElement::Refinement(name, fields) => {
                        if elem.span.contains(&offset) {
                            // Check if on the binding name part
                            // For now, just resolve the binding
                            if let Some(inst) = env.get_instance(name) {
                                return Some(inst.span.clone());
                            }
                        }
                        for field in fields {
                            if let Some(span) = find_in_value(&field.node.value, env, offset) {
                                return Some(span);
                            }
                        }
                    }
                }
            }
            None
        }
        Value::Variant(_, inner) => find_in_value(inner, env, offset),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::resolve::resolve;
    use std::path::Path;

    fn test_definition(src: &str, offset: usize) -> Option<Span> {
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let (env, _) = resolve(&file, Path::new("test.ilk"));
        find_definition(&file, &env, offset)
    }

    #[test]
    fn test_goto_named_type() {
        let src = "meta Foo = {x Int}\nmeta Bar = Foo";
        // "Foo" in "Bar = Foo" starts at position 30
        let span = test_definition(src, 30);
        assert!(span.is_some());
        // Should point to Foo definition
        assert_eq!(span.unwrap().start, 0);
    }

    #[test]
    fn test_goto_instance_type() {
        let src = "meta Foo = {x Int}\nfoo = Foo {x Int}";
        // "Foo" in "foo = Foo" starts around position 25
        let span = test_definition(src, 25);
        assert!(span.is_some());
    }
}
