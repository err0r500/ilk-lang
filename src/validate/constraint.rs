use crate::error::Diagnostic;
use crate::ilk::ast::*;
use crate::kli::ast::*;
use crate::span::S;
use crate::validate::structural::ValidationContext;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
enum Value {
    Bool(bool),
    Int(i64),
    String(String),
    List(Vec<Value>),
    Set(HashSet<String>),
    Struct(HashMap<String, Value>),
    BindingRef(String),
}

pub fn validate_constraints(
    ctx: &ValidationContext,
    kli: &KliFile,
    errors: &mut Vec<Diagnostic>,
) {
    for binding in &kli.bindings {
        let type_name = &binding.node.type_name.node;
        if let Some(block) = ctx.env.get(type_name) {
            validate_binding_constraints(ctx, binding, block, errors);
        }
    }
}

fn validate_binding_constraints(
    ctx: &ValidationContext,
    binding: &S<Binding>,
    block: &S<Block>,
    errors: &mut Vec<Diagnostic>,
) {
    // Find @constraint annotations in the block
    let constraints: Vec<_> = find_constraints_in_type(&block.node.body);

    for constraint in constraints {
        let env = build_eval_env(ctx, binding);
        let assocs = build_assoc_map(ctx, binding);

        match eval_constraint(&constraint.node, &env, &assocs, ctx) {
            Ok(Value::Bool(true)) => {}
            Ok(Value::Bool(false)) => {
                errors.push(Diagnostic::error(
                    constraint.span.clone(),
                    format!(
                        "Constraint failed for binding '{}'",
                        binding.node.name.node
                    ),
                    ctx.path,
                ));
            }
            Ok(_) => {
                errors.push(Diagnostic::error(
                    constraint.span.clone(),
                    "Constraint must evaluate to boolean",
                    ctx.path,
                ));
            }
            Err(msg) => {
                errors.push(Diagnostic::error(
                    constraint.span.clone(),
                    format!("Constraint evaluation error: {}", msg),
                    ctx.path,
                ));
            }
        }
    }
}

fn find_constraints_in_type(ty: &S<TypeExpr>) -> Vec<S<ConstraintExpr>> {
    let mut constraints = Vec::new();

    match &ty.node {
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            for field in fields {
                for ann in &field.node.annotations {
                    if let Annotation::Constraint(c) = &ann.node {
                        constraints.push(c.clone());
                    }
                }
            }
        }
        TypeExpr::Intersection(left, right) => {
            constraints.extend(find_constraints_in_type(left));
            constraints.extend(find_constraints_in_type(right));
        }
        _ => {}
    }

    constraints
}

fn build_eval_env(ctx: &ValidationContext, binding: &S<Binding>) -> HashMap<String, Value> {
    let mut env = HashMap::new();

    if let KliValue::Struct(fields) = &binding.node.body.node {
        for field in fields {
            let name = &field.node.name.node;
            let value = kli_value_to_eval_value(&field.node.value, ctx);
            env.insert(name.clone(), value);
        }
    }

    env
}

fn build_assoc_map(_ctx: &ValidationContext, binding: &S<Binding>) -> HashSet<String> {
    binding
        .node
        .assocs
        .iter()
        .map(|a| a.node.clone())
        .collect()
}

fn kli_value_to_eval_value(value: &S<KliValue>, ctx: &ValidationContext) -> Value {
    match &value.node {
        KliValue::LitString(s) => Value::String(s.clone()),
        KliValue::LitInt(n) => Value::Int(*n),
        KliValue::LitBool(b) => Value::Bool(*b),
        KliValue::TypeRef(t) => Value::String(t.clone()),
        KliValue::BindingRef(name) => Value::BindingRef(name.clone()),
        KliValue::Struct(fields) => {
            let mut map = HashMap::new();
            for field in fields {
                let name = &field.node.name.node;
                let val = kli_value_to_eval_value(&field.node.value, ctx);
                map.insert(name.clone(), val);
            }
            Value::Struct(map)
        }
        KliValue::List(elements) => {
            let vals: Vec<_> = elements
                .iter()
                .map(|e| match &e.node {
                    KliListElement::Value(v) => {
                        let spanned = S::new(v.clone(), e.span.clone());
                        kli_value_to_eval_value(&spanned, ctx)
                    }
                    KliListElement::BindingRef(name) => Value::BindingRef(name.clone()),
                    KliListElement::Refinement(name, _) => Value::BindingRef(name.clone()),
                })
                .collect();
            Value::List(vals)
        }
        KliValue::Variant(_, body) => kli_value_to_eval_value(body, ctx),
    }
}

fn eval_constraint(
    expr: &ConstraintExpr,
    env: &HashMap<String, Value>,
    assocs: &HashSet<String>,
    ctx: &ValidationContext,
) -> Result<Value, String> {
    match expr {
        ConstraintExpr::Bool(b) => Ok(Value::Bool(*b)),
        ConstraintExpr::Int(n) => Ok(Value::Int(*n)),

        ConstraintExpr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Unknown variable: {}", name)),

        ConstraintExpr::FieldAccess(obj, field) => {
            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                Value::Struct(map) => map
                    .get(field)
                    .cloned()
                    .ok_or_else(|| format!("Unknown field: {}", field)),
                Value::BindingRef(name) => {
                    // Look up the binding and get its field
                    if let Some(binding) = ctx.bindings.get(&name) {
                        if let KliValue::Struct(fields) = &binding.node.body.node {
                            if let Some(f) = fields.iter().find(|f| &f.node.name.node == field) {
                                return Ok(kli_value_to_eval_value(&f.node.value, ctx));
                            }
                        }
                    }
                    Err(format!("Cannot access field {} on {}", field, name))
                }
                _ => Err(format!("Cannot access field {} on non-struct", field)),
            }
        }

        ConstraintExpr::ForAll(col, var, body) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let Value::List(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    // Get assocs for this item if it's a binding ref
                    let item_assocs = match item {
                        Value::BindingRef(name) => {
                            if let Some(binding) = ctx.bindings.get(name) {
                                binding
                                    .node
                                    .assocs
                                    .iter()
                                    .map(|a| a.node.clone())
                                    .collect()
                            } else {
                                HashSet::new()
                            }
                        }
                        _ => HashSet::new(),
                    };

                    let result = eval_constraint(&body.node, &inner_env, &item_assocs, ctx)?;
                    if let Value::Bool(false) = result {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(true))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::ForAllExpr(col_expr, var, body) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            if let Value::Set(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), Value::String(item));

                    let result = eval_constraint(&body.node, &inner_env, assocs, ctx)?;
                    if let Value::Bool(false) = result {
                        return Ok(Value::Bool(false));
                    }
                }
                Ok(Value::Bool(true))
            } else {
                Err("ForAllExpr collection must be a set".to_string())
            }
        }

        ConstraintExpr::Exists(col, var, body) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let Value::List(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let item_assocs = match item {
                        Value::BindingRef(name) => {
                            if let Some(binding) = ctx.bindings.get(name) {
                                binding
                                    .node
                                    .assocs
                                    .iter()
                                    .map(|a| a.node.clone())
                                    .collect()
                            } else {
                                HashSet::new()
                            }
                        }
                        _ => HashSet::new(),
                    };

                    let result = eval_constraint(&body.node, &inner_env, &item_assocs, ctx)?;
                    if let Value::Bool(true) = result {
                        return Ok(Value::Bool(true));
                    }
                }
                Ok(Value::Bool(false))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Unique(col, var, body) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let Value::List(items) = col_val {
                let mut seen = HashSet::new();
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let result = eval_constraint(&body.node, &inner_env, assocs, ctx)?;
                    let key = value_to_string(&result);
                    if seen.contains(&key) {
                        return Ok(Value::Bool(false));
                    }
                    seen.insert(key);
                }
                Ok(Value::Bool(true))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Count(col) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let Value::List(items) = col_val {
                Ok(Value::Int(items.len() as i64))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Assoc(obj, tag) => {
            // e.assoc(t) - check if e has t as an association
            let tag_val = eval_constraint(&tag.node, env, assocs, ctx)?;
            let tag_name = match tag_val {
                Value::BindingRef(name) => name,
                _ => return Err("assoc tag must be a binding reference".to_string()),
            };

            // Get the object's associations
            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                Value::BindingRef(name) => {
                    if let Some(binding) = ctx.bindings.get(&name) {
                        let has = binding.node.assocs.iter().any(|a| a.node == tag_name);
                        Ok(Value::Bool(has))
                    } else {
                        Err(format!("Unknown binding: {}", name))
                    }
                }
                _ => {
                    // Check current context's assocs
                    Ok(Value::Bool(assocs.contains(&tag_name)))
                }
            }
        }

        ConstraintExpr::TemplateVars(expr) => {
            let val = eval_constraint(&expr.node, env, assocs, ctx)?;
            if let Value::String(s) = val {
                let vars = extract_template_vars(&s);
                Ok(Value::Set(vars))
            } else {
                Err("templateVars requires a string".to_string())
            }
        }

        ConstraintExpr::Keys(expr) => {
            let val = eval_constraint(&expr.node, env, assocs, ctx)?;
            match val {
                Value::Struct(map) => {
                    let keys: HashSet<_> = map.keys().cloned().collect();
                    Ok(Value::Set(keys))
                }
                Value::BindingRef(name) => {
                    if let Some(binding) = ctx.bindings.get(&name) {
                        if let KliValue::Struct(fields) = &binding.node.body.node {
                            let keys: HashSet<_> =
                                fields.iter().map(|f| f.node.name.node.clone()).collect();
                            return Ok(Value::Set(keys));
                        }
                    }
                    Err(format!("Cannot get keys of {}", name))
                }
                _ => Err("keys requires a struct".to_string()),
            }
        }

        ConstraintExpr::And(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
                _ => Err("&& requires boolean operands".to_string()),
            }
        }

        ConstraintExpr::Or(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
                _ => Err("|| requires boolean operands".to_string()),
            }
        }

        ConstraintExpr::Not(inner) => {
            let v = eval_constraint(&inner.node, env, assocs, ctx)?;
            match v {
                Value::Bool(b) => Ok(Value::Bool(!b)),
                _ => Err("! requires boolean operand".to_string()),
            }
        }

        ConstraintExpr::Eq(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            Ok(Value::Bool(values_equal(&l, &r)))
        }

        ConstraintExpr::Ne(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            Ok(Value::Bool(!values_equal(&l, &r)))
        }

        ConstraintExpr::In(elem, set) => {
            let e = eval_constraint(&elem.node, env, assocs, ctx)?;
            let s = eval_constraint(&set.node, env, assocs, ctx)?;

            match (e, s) {
                (Value::String(key), Value::Set(set)) => Ok(Value::Bool(set.contains(&key))),
                (Value::BindingRef(key), Value::Set(set)) => Ok(Value::Bool(set.contains(&key))),
                _ => Err("in requires element and set".to_string()),
            }
        }

        ConstraintExpr::Lt(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
                _ => Err("< requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Le(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
                _ => Err("<= requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Gt(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
                _ => Err("> requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Ge(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
                _ => Err(">= requires integer operands".to_string()),
            }
        }
    }
}

fn extract_template_vars(s: &str) -> HashSet<String> {
    let mut vars = HashSet::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut var = String::new();
            for c in chars.by_ref() {
                if c == '}' {
                    break;
                }
                var.push(c);
            }
            if !var.is_empty() {
                vars.insert(var);
            }
        }
    }

    vars
}

fn value_to_string(val: &Value) -> String {
    match val {
        Value::Bool(b) => b.to_string(),
        Value::Int(n) => n.to_string(),
        Value::String(s) => s.clone(),
        Value::BindingRef(name) => name.clone(),
        Value::List(items) => format!(
            "[{}]",
            items
                .iter()
                .map(value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Set(items) => format!("{{{}}}", items.iter().cloned().collect::<Vec<_>>().join(", ")),
        Value::Struct(map) => format!(
            "{{{}}}",
            map.iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Int(x), Value::Int(y)) => x == y,
        (Value::String(x), Value::String(y)) => x == y,
        (Value::BindingRef(x), Value::BindingRef(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ilk::{parse_ilk, resolve};
    use crate::kli::parse_kli;
    use std::path::Path;

    fn validate_constraints_pair(ilk_src: &str, kli_src: &str) -> Vec<Diagnostic> {
        let ilk = parse_ilk(ilk_src, Path::new("test.ilk")).unwrap();
        let env = resolve(&ilk, Path::new("test.ilk")).unwrap();
        let kli = parse_kli(kli_src, Path::new("test.kli")).unwrap();
        let ctx = ValidationContext::new(&env, &kli, Path::new("test.kli"));
        let mut errors = Vec::new();
        validate_constraints(&ctx, &kli, &mut errors);
        errors
    }

    #[test]
    fn test_forall_true() {
        let errors = validate_constraints_pair(
            "@main\nFoo {\n  @constraint forall(items, i => true)\n  items []Bar\n}\nBar {x Int}",
            "b1 = Bar {x Int}\nb2 = Bar {x Int}\nfoo = Foo {items [b1, b2]}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_forall_empty() {
        let errors = validate_constraints_pair(
            "@main\nFoo {\n  @constraint forall(items, i => true)\n  items []Bar\n}\nBar {x Int}",
            "foo = Foo {items []}",
        );
        assert!(errors.is_empty(), "{:?}", errors); // vacuously true
    }

    #[test]
    fn test_count() {
        let errors = validate_constraints_pair(
            "@main\nFoo {\n  @constraint count(items) >= 1\n  items []Bar\n}\nBar {x Int}",
            "b1 = Bar {x Int}\nfoo = Foo {items [b1]}",
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_count_fail() {
        let errors = validate_constraints_pair(
            "@main\nFoo {\n  @constraint count(items) >= 1\n  items []Bar\n}\nBar {x Int}",
            "foo = Foo {items []}",
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_template_vars() {
        let vars = extract_template_vars("/users/{id}");
        assert!(vars.contains("id"));
        assert_eq!(vars.len(), 1);
    }

    #[test]
    fn test_template_vars_multiple() {
        let vars = extract_template_vars("/users/{id}/posts/{postId}");
        assert!(vars.contains("id"));
        assert!(vars.contains("postId"));
        assert_eq!(vars.len(), 2);
    }
}
