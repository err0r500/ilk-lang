use crate::ast::*;
use crate::resolve::TypeEnv;
use crate::span::S;

/// Get hover info for the symbol at the given offset
pub fn hover_info(file: &File, env: &TypeEnv, offset: usize) -> Option<String> {
    // Check type declarations (for references within type bodies)
    for item in &file.items {
        if let Item::TypeDecl(decl) = &item.node {
            // Hovering on the type name itself
            if decl.name.span.contains(&offset) {
                return Some(format_type_hover(decl));
            }
            if let Some(info) = hover_in_type_expr(&decl.body, env, offset) {
                return Some(info);
            }
        }
    }

    // Check instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            // Hovering on instance name
            if inst.name.span.contains(&offset) {
                return Some(format_instance_hover(inst));
            }
            // Hovering on instance type_name
            if inst.type_name.span.contains(&offset) {
                if let Some(decl) = env.get_type(&inst.type_name.node) {
                    return Some(format_type_hover(&decl.node));
                }
            }
            // Within instance body
            if let Some(info) = hover_in_value(&inst.body, env, offset) {
                return Some(info);
            }
        }
    }

    None
}

fn hover_in_type_expr(ty: &S<TypeExpr>, env: &TypeEnv, offset: usize) -> Option<String> {
    if !ty.span.contains(&offset) {
        return None;
    }

    match &ty.node {
        TypeExpr::Named(name) | TypeExpr::Reference(name) | TypeExpr::RefinableRef(name) => {
            env.get_type(name).map(|t| format_type_hover(&t.node))
        }
        TypeExpr::Concrete(inner) | TypeExpr::List(_, inner) => {
            hover_in_type_expr(inner, env, offset)
        }
        TypeExpr::Union(variants) => {
            for v in variants {
                if let Some(info) = hover_in_type_expr(v, env, offset) {
                    return Some(info);
                }
            }
            None
        }
        TypeExpr::Intersection(left, right) => {
            hover_in_type_expr(left, env, offset)
                .or_else(|| hover_in_type_expr(right, env, offset))
        }
        TypeExpr::Struct(kind) => {
            let fields = match kind {
                StructKind::Closed(f) | StructKind::Open(f) => f,
                StructKind::Anonymous(types) => {
                    for ty in types.iter().flatten() {
                        if let Some(info) = hover_in_type_expr(ty, env, offset) {
                            return Some(info);
                        }
                    }
                    return None;
                }
            };
            for field in fields {
                // Hover on field name shows field type
                if field.node.name.span.contains(&offset) {
                    return Some(format!(
                        "**{}**: {}",
                        field.node.name.node,
                        type_expr_to_string(&field.node.ty.node)
                    ));
                }
                if let Some(info) = hover_in_type_expr(&field.node.ty, env, offset) {
                    return Some(info);
                }
            }
            None
        }
        _ => None,
    }
}

fn hover_in_value(val: &S<Value>, env: &TypeEnv, offset: usize) -> Option<String> {
    if !val.span.contains(&offset) {
        return None;
    }

    match &val.node {
        Value::TypeRef(name) => env.get_type(name).map(|t| format_type_hover(&t.node)),
        Value::BindingRef(name) => env.get_instance(name).map(|i| format_instance_hover(&i.node)),
        Value::Struct(fields) => {
            for field in fields {
                // Field name hover
                if field.node.name.span.contains(&offset) {
                    return Some(format!(
                        "**{}**: {}",
                        field.node.name.node,
                        value_to_type_string(&field.node.value.node)
                    ));
                }
                if let Some(info) = hover_in_value(&field.node.value, env, offset) {
                    return Some(info);
                }
            }
            None
        }
        Value::List(elements) => {
            for elem in elements {
                if !elem.span.contains(&offset) {
                    continue;
                }
                match &elem.node {
                    ListElement::Value(v) => {
                        let v_spanned = S::new(v.clone(), elem.span.clone());
                        if let Some(info) = hover_in_value(&v_spanned, env, offset) {
                            return Some(info);
                        }
                    }
                    ListElement::BindingRef(name) => {
                        return env.get_instance(name).map(|i| format_instance_hover(&i.node));
                    }
                    ListElement::Refinement(name, fields) => {
                        if let Some(inst) = env.get_instance(name) {
                            // Check if hovering on the name part (rough heuristic)
                            let name_end_approx = elem.span.start + name.len();
                            if offset < name_end_approx {
                                return Some(format_instance_hover(&inst.node));
                            }
                        }
                        for field in fields {
                            if let Some(info) = hover_in_value(&field.node.value, env, offset) {
                                return Some(info);
                            }
                        }
                    }
                }
            }
            None
        }
        Value::Variant(_, inner) => hover_in_value(inner, env, offset),
        _ => None,
    }
}

fn format_type_hover(decl: &TypeDecl) -> String {
    let mut result = format!("**type {}**\n\n", decl.name.node);
    result.push_str(&format!("```ilk\n{}\n```", type_expr_to_string(&decl.body.node)));

    // Add @doc if present
    for ann in &decl.annotations {
        if let Annotation::Doc(doc) = &ann.node {
            result.push_str("\n\n---\n\n");
            result.push_str(doc);
        }
    }

    result
}

fn format_instance_hover(inst: &Instance) -> String {
    let mut result = format!("**{}: {}**", inst.name.node, inst.type_name.node);

    if let Some(doc) = &inst.doc {
        result.push_str("\n\n---\n\n");
        result.push_str(doc);
    }

    // Add @doc from annotations
    for ann in &inst.annotations {
        if let Annotation::Doc(doc) = &ann.node {
            result.push_str("\n\n---\n\n");
            result.push_str(doc);
        }
    }

    result
}

fn type_expr_to_string(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Base(b) => format!("{:?}", b),
        TypeExpr::Named(n) => n.clone(),
        TypeExpr::RefinableRef(n) => format!("-{}", n),
        TypeExpr::LitString(s) => format!("\"{}\"", s),
        TypeExpr::LitInt(i) => i.to_string(),
        TypeExpr::LitBool(b) => b.to_string(),
        TypeExpr::Reference(r) => format!("&{}", r),
        TypeExpr::Concrete(inner) => format!("*{}", type_expr_to_string(&inner.node)),
        TypeExpr::List(card, inner) => {
            let card_str = match card {
                Cardinality::Any => "".to_string(),
                Cardinality::Exact(n) => n.to_string(),
                Cardinality::AtLeast(n) => format!("{}..", n),
                Cardinality::AtMost(n) => format!("..{}", n),
                Cardinality::Range(a, b) => format!("{}..{}", a, b),
            };
            format!("[{}{}]", card_str, type_expr_to_string(&inner.node))
        }
        TypeExpr::Union(variants) => variants
            .iter()
            .map(|v| type_expr_to_string(&v.node))
            .collect::<Vec<_>>()
            .join(" | "),
        TypeExpr::Intersection(left, right) => format!(
            "{} & {}",
            type_expr_to_string(&left.node),
            type_expr_to_string(&right.node)
        ),
        TypeExpr::Struct(kind) => match kind {
            StructKind::Closed(fields) => {
                let field_strs: Vec<String> = fields
                    .iter()
                    .map(|f| {
                        format!(
                            "{}{} {}",
                            f.node.name.node,
                            if f.node.optional { "?" } else { "" },
                            type_expr_to_string(&f.node.ty.node)
                        )
                    })
                    .collect();
                format!("{{{}}}", field_strs.join(", "))
            }
            StructKind::Open(fields) => {
                if fields.is_empty() {
                    "{...}".to_string()
                } else {
                    let field_strs: Vec<String> = fields
                        .iter()
                        .map(|f| {
                            format!(
                                "{}{} {}",
                                f.node.name.node,
                                if f.node.optional { "?" } else { "" },
                                type_expr_to_string(&f.node.ty.node)
                            )
                        })
                        .collect();
                    format!("{{..., {}}}", field_strs.join(", "))
                }
            }
            StructKind::Anonymous(types) => {
                let type_strs: Vec<String> = types
                    .iter()
                    .map(|t| t.as_ref().map(|t| type_expr_to_string(&t.node)).unwrap_or_else(|| "_".to_string()))
                    .collect();
                format!("{{{}}}", type_strs.join(", "))
            }
        },
    }
}

fn value_to_type_string(val: &Value) -> String {
    match val {
        Value::TypeRef(name) => name.clone(),
        Value::LitString(_) => "String".to_string(),
        Value::LitInt(_) => "Int".to_string(),
        Value::LitBool(_) => "Bool".to_string(),
        Value::BindingRef(name) => format!("&{}", name),
        Value::Struct(_) => "{...}".to_string(),
        Value::List(_) => "[...]".to_string(),
        Value::Variant(name, _) => name.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::resolve::resolve;
    use std::path::Path;

    fn test_hover(src: &str, offset: usize) -> Option<String> {
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        hover_info(&file, &env, offset)
    }

    #[test]
    fn test_hover_type_name() {
        let src = "type Foo = {x Int}";
        // "Foo" starts at position 5
        let info = test_hover(src, 5);
        assert!(info.is_some());
        assert!(info.unwrap().contains("type Foo"));
    }

    #[test]
    fn test_hover_instance_name() {
        let src = "type Foo = {x Int}\nfoo = Foo {x Int}";
        // "foo" starts at position 19
        let info = test_hover(src, 19);
        assert!(info.is_some());
        assert!(info.unwrap().contains("foo: Foo"));
    }
}
