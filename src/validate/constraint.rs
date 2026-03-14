use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::S;
use crate::validate::structural::ValidationContext;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
enum EvalValue {
    Bool(bool),
    Int(i64),
    String(String),
    List(Vec<EvalValue>),
    Set(HashSet<String>),
    Struct(HashMap<String, EvalValue>),
    BindingRef(String),
}

pub fn validate_constraints(
    ctx: &ValidationContext,
    file: &File,
    errors: &mut Vec<Diagnostic>,
) {
    for inst in file.instances() {
        let type_name = &inst.type_name.node;
        if let Some(type_decl) = ctx.env.get_type(type_name) {
            validate_instance_constraints(ctx, inst, &type_decl.node, errors);
        }
    }
}

fn validate_instance_constraints(
    ctx: &ValidationContext,
    inst: &Instance,
    type_decl: &TypeDecl,
    errors: &mut Vec<Diagnostic>,
) {
    let constraints: Vec<_> = find_constraints_in_type(&type_decl.body);

    for constraint in constraints {
        let env = build_eval_env(ctx, inst);
        let assocs = build_assoc_map(inst);

        match eval_constraint(&constraint.node, &env, &assocs, ctx) {
            Ok(EvalValue::Bool(true)) => {}
            Ok(EvalValue::Bool(false)) => {
                errors.push(Diagnostic::error(
                    constraint.span.clone(),
                    format!(
                        "Constraint failed for instance '{}'",
                        inst.name.node
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

    // Recursively validate constraints on nested structures
    validate_nested_constraints(
        &inst.body,
        &type_decl.body,
        &inst.name.node,
        ctx,
        errors,
    );
}

fn validate_nested_constraints(
    value: &S<Value>,
    ty: &S<TypeExpr>,
    parent_path: &str,
    ctx: &ValidationContext,
    errors: &mut Vec<Diagnostic>,
) {
    match (&value.node, &ty.node) {
        (Value::List(elements), TypeExpr::List(_, elem_ty)) => {
            let type_decl = match &elem_ty.node {
                TypeExpr::Named(name) => ctx.env.get_type(name),
                _ => None,
            };

            if let Some(type_decl) = type_decl {
                let constraints = find_constraints_in_type(&type_decl.node.body);
                if !constraints.is_empty() {
                    for (i, elem) in elements.iter().enumerate() {
                        if let ListElement::Value(Value::Struct(fields)) = &elem.node {
                            let env = build_env_from_fields(fields, ctx);
                            let assocs = HashSet::new();

                            for constraint in &constraints {
                                match eval_constraint(&constraint.node, &env, &assocs, ctx) {
                                    Ok(EvalValue::Bool(true)) => {}
                                    Ok(EvalValue::Bool(false)) => {
                                        errors.push(Diagnostic::error(
                                            elem.span.clone(),
                                            format!(
                                                "Constraint failed for inline {} at {}[{}]",
                                                type_decl.node.name.node, parent_path, i
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

                            let inline_value = S::new(Value::Struct(fields.clone()), elem.span.clone());
                            validate_nested_constraints(
                                &inline_value,
                                &type_decl.node.body,
                                &format!("{}[{}]", parent_path, i),
                                ctx,
                                errors,
                            );
                        }
                    }
                }
            }
        }

        (Value::Struct(val_fields), TypeExpr::Struct(kind)) => {
            let type_fields = match kind {
                StructKind::Closed(fields) | StructKind::Open(fields) => fields,
                _ => return,
            };

            for val_field in val_fields {
                let name = &val_field.node.name.node;
                if let Some(type_field) = type_fields.iter().find(|f| &f.node.name.node == name) {
                    validate_nested_constraints(
                        &val_field.node.value,
                        &type_field.node.ty,
                        &format!("{}.{}", parent_path, name),
                        ctx,
                        errors,
                    );
                }
            }
        }

        (_, TypeExpr::Intersection(left, right)) => {
            validate_nested_constraints(value, left, parent_path, ctx, errors);
            validate_nested_constraints(value, right, parent_path, ctx, errors);
        }

        _ => {}
    }
}

fn build_env_from_fields(
    fields: &[S<InstanceField>],
    ctx: &ValidationContext,
) -> HashMap<String, EvalValue> {
    let mut env = HashMap::new();
    for field in fields {
        let name = &field.node.name.node;
        let value = value_to_eval_value(&field.node.value, ctx);
        env.insert(name.clone(), value);
    }
    env
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

fn build_eval_env(ctx: &ValidationContext, inst: &Instance) -> HashMap<String, EvalValue> {
    let mut env = HashMap::new();

    if let Value::Struct(fields) = &inst.body.node {
        for field in fields {
            let name = &field.node.name.node;
            let value = value_to_eval_value(&field.node.value, ctx);
            env.insert(name.clone(), value);
        }
    }

    env
}

fn build_assoc_map(inst: &Instance) -> HashSet<String> {
    inst.assocs
        .iter()
        .map(|a| a.node.clone())
        .collect()
}

fn value_to_eval_value(value: &S<Value>, ctx: &ValidationContext) -> EvalValue {
    match &value.node {
        Value::LitString(s) => EvalValue::String(s.clone()),
        Value::LitInt(n) => EvalValue::Int(*n),
        Value::LitBool(b) => EvalValue::Bool(*b),
        Value::TypeRef(t) => EvalValue::String(t.clone()),
        Value::BindingRef(name) => EvalValue::BindingRef(name.clone()),
        Value::Struct(fields) => {
            let mut map = HashMap::new();
            for field in fields {
                let name = &field.node.name.node;
                let val = value_to_eval_value(&field.node.value, ctx);
                map.insert(name.clone(), val);
            }
            EvalValue::Struct(map)
        }
        Value::List(elements) => {
            let vals: Vec<_> = elements
                .iter()
                .map(|e| match &e.node {
                    ListElement::Value(v) => {
                        let spanned = S::new(v.clone(), e.span.clone());
                        value_to_eval_value(&spanned, ctx)
                    }
                    ListElement::BindingRef(name) => EvalValue::BindingRef(name.clone()),
                    ListElement::Refinement(name, _) => EvalValue::BindingRef(name.clone()),
                })
                .collect();
            EvalValue::List(vals)
        }
        Value::Variant(_, body) => value_to_eval_value(body, ctx),
    }
}

fn eval_constraint(
    expr: &ConstraintExpr,
    env: &HashMap<String, EvalValue>,
    assocs: &HashSet<String>,
    ctx: &ValidationContext,
) -> Result<EvalValue, String> {
    match expr {
        ConstraintExpr::Bool(b) => Ok(EvalValue::Bool(*b)),
        ConstraintExpr::Int(n) => Ok(EvalValue::Int(*n)),

        ConstraintExpr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| format!("Unknown variable: {}", name)),

        ConstraintExpr::FieldAccess(obj, field) => {
            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                EvalValue::Struct(map) => map
                    .get(field)
                    .cloned()
                    .ok_or_else(|| format!("Unknown field: {}", field)),
                EvalValue::BindingRef(name) => {
                    if let Some(inst) = ctx.get_instance(&name) {
                        if let Value::Struct(fields) = &inst.body.node {
                            if let Some(f) = fields.iter().find(|f| &f.node.name.node == field) {
                                return Ok(value_to_eval_value(&f.node.value, ctx));
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

            if let EvalValue::List(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let item_assocs = match item {
                        EvalValue::BindingRef(name) => {
                            if let Some(inst) = ctx.get_instance(name) {
                                inst.assocs
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
                    if let EvalValue::Bool(false) = result {
                        return Ok(EvalValue::Bool(false));
                    }
                }
                Ok(EvalValue::Bool(true))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::ForAllExpr(col_expr, var, body) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            if let EvalValue::Set(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), EvalValue::String(item));

                    let result = eval_constraint(&body.node, &inner_env, assocs, ctx)?;
                    if let EvalValue::Bool(false) = result {
                        return Ok(EvalValue::Bool(false));
                    }
                }
                Ok(EvalValue::Bool(true))
            } else {
                Err("ForAllExpr collection must be a set".to_string())
            }
        }

        ConstraintExpr::Exists(col, var, body) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let EvalValue::List(items) = col_val {
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let item_assocs = match item {
                        EvalValue::BindingRef(name) => {
                            if let Some(inst) = ctx.get_instance(name) {
                                inst.assocs
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
                    if let EvalValue::Bool(true) = result {
                        return Ok(EvalValue::Bool(true));
                    }
                }
                Ok(EvalValue::Bool(false))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Unique(col, var, body) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let EvalValue::List(items) = col_val {
                let mut seen = HashSet::new();
                for item in items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let result = eval_constraint(&body.node, &inner_env, assocs, ctx)?;
                    let key = eval_value_to_string(&result);
                    if seen.contains(&key) {
                        return Ok(EvalValue::Bool(false));
                    }
                    seen.insert(key);
                }
                Ok(EvalValue::Bool(true))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Count(col) => {
            let col_val = env
                .get(col)
                .ok_or_else(|| format!("Unknown collection: {}", col))?;

            if let EvalValue::List(items) = col_val {
                Ok(EvalValue::Int(items.len() as i64))
            } else {
                Err(format!("{} is not a list", col))
            }
        }

        ConstraintExpr::Assoc(obj, tag) => {
            let tag_val = eval_constraint(&tag.node, env, assocs, ctx)?;
            let tag_name = match tag_val {
                EvalValue::BindingRef(name) => name,
                _ => return Err("assoc tag must be a binding reference".to_string()),
            };

            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                EvalValue::BindingRef(name) => {
                    if let Some(inst) = ctx.get_instance(&name) {
                        let has = inst.assocs.iter().any(|a| a.node == tag_name);
                        Ok(EvalValue::Bool(has))
                    } else {
                        Err(format!("Unknown instance: {}", name))
                    }
                }
                _ => {
                    Ok(EvalValue::Bool(assocs.contains(&tag_name)))
                }
            }
        }

        ConstraintExpr::TemplateVars(expr) => {
            let val = eval_constraint(&expr.node, env, assocs, ctx)?;
            if let EvalValue::String(s) = val {
                let vars = extract_template_vars(&s);
                Ok(EvalValue::Set(vars))
            } else {
                Err("templateVars requires a string".to_string())
            }
        }

        ConstraintExpr::Keys(expr) => {
            let val = eval_constraint(&expr.node, env, assocs, ctx)?;
            match val {
                EvalValue::Struct(map) => {
                    let keys: HashSet<_> = map.keys().cloned().collect();
                    Ok(EvalValue::Set(keys))
                }
                EvalValue::BindingRef(name) => {
                    if let Some(inst) = ctx.get_instance(&name) {
                        if let Value::Struct(fields) = &inst.body.node {
                            let keys: HashSet<_> =
                                fields.iter().map(|f| f.node.name.node.clone()).collect();
                            return Ok(EvalValue::Set(keys));
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
                (EvalValue::Bool(a), EvalValue::Bool(b)) => Ok(EvalValue::Bool(a && b)),
                _ => Err("&& requires boolean operands".to_string()),
            }
        }

        ConstraintExpr::Or(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (EvalValue::Bool(a), EvalValue::Bool(b)) => Ok(EvalValue::Bool(a || b)),
                _ => Err("|| requires boolean operands".to_string()),
            }
        }

        ConstraintExpr::Not(inner) => {
            let v = eval_constraint(&inner.node, env, assocs, ctx)?;
            match v {
                EvalValue::Bool(b) => Ok(EvalValue::Bool(!b)),
                _ => Err("! requires boolean operand".to_string()),
            }
        }

        ConstraintExpr::Eq(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            Ok(EvalValue::Bool(eval_values_equal(&l, &r)))
        }

        ConstraintExpr::Ne(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            Ok(EvalValue::Bool(!eval_values_equal(&l, &r)))
        }

        ConstraintExpr::In(elem, set) => {
            let e = eval_constraint(&elem.node, env, assocs, ctx)?;
            let s = eval_constraint(&set.node, env, assocs, ctx)?;

            match (e, s) {
                (EvalValue::String(key), EvalValue::Set(set)) => Ok(EvalValue::Bool(set.contains(&key))),
                (EvalValue::BindingRef(key), EvalValue::Set(set)) => Ok(EvalValue::Bool(set.contains(&key))),
                _ => Err("in requires element and set".to_string()),
            }
        }

        ConstraintExpr::Lt(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (EvalValue::Int(a), EvalValue::Int(b)) => Ok(EvalValue::Bool(a < b)),
                _ => Err("< requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Le(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (EvalValue::Int(a), EvalValue::Int(b)) => Ok(EvalValue::Bool(a <= b)),
                _ => Err("<= requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Gt(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (EvalValue::Int(a), EvalValue::Int(b)) => Ok(EvalValue::Bool(a > b)),
                _ => Err("> requires integer operands".to_string()),
            }
        }

        ConstraintExpr::Ge(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            let r = eval_constraint(&right.node, env, assocs, ctx)?;
            match (l, r) {
                (EvalValue::Int(a), EvalValue::Int(b)) => Ok(EvalValue::Bool(a >= b)),
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

fn eval_value_to_string(val: &EvalValue) -> String {
    match val {
        EvalValue::Bool(b) => b.to_string(),
        EvalValue::Int(n) => n.to_string(),
        EvalValue::String(s) => s.clone(),
        EvalValue::BindingRef(name) => name.clone(),
        EvalValue::List(items) => format!(
            "[{}]",
            items
                .iter()
                .map(eval_value_to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        EvalValue::Set(items) => format!("{{{}}}", items.iter().cloned().collect::<Vec<_>>().join(", ")),
        EvalValue::Struct(map) => format!(
            "{{{}}}",
            map.iter()
                .map(|(k, v)| format!("{}: {}", k, eval_value_to_string(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn eval_values_equal(a: &EvalValue, b: &EvalValue) -> bool {
    match (a, b) {
        (EvalValue::Bool(x), EvalValue::Bool(y)) => x == y,
        (EvalValue::Int(x), EvalValue::Int(y)) => x == y,
        (EvalValue::String(x), EvalValue::String(y)) => x == y,
        (EvalValue::BindingRef(x), EvalValue::BindingRef(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;
    use crate::resolve::resolve;
    use std::path::Path;

    fn validate_constraints_src(src: &str) -> Vec<Diagnostic> {
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        validate_constraints(&ctx, &file, &mut errors);
        errors
    }

    #[test]
    fn test_forall_true() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint forall(items, i => true)
  items []Bar
}
b1 = Bar {x Int}
b2 = Bar {x Int}
foo = Foo {items [b1, b2]}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_forall_empty() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint forall(items, i => true)
  items []Bar
}
foo = Foo {items []}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors); // vacuously true
    }

    #[test]
    fn test_count() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint count(items) >= 1
  items []Bar
}
b1 = Bar {x Int}
foo = Foo {items [b1]}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_count_fail() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint count(items) >= 1
  items []Bar
}
foo = Foo {items []}
"#,
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

    #[test]
    fn test_nested_constraint_pass() {
        let errors = validate_constraints_src(
            r#"
type Tag = {_ String}

@assoc [Tag]
type Event = {...}

type QueryItem = {
    @constraint forall(tags, t => forall(events, e => e.assoc(t)))
    events []Event
    tags []Tag
}

type Container = { items []QueryItem }

tag1 = Tag {x String}
ev1 = Event<tag1> {a String}
ev2 = Event<tag1> {b String}
container = Container {
    items [
        { events [ev1, ev2], tags [tag1] }
    ]
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_nested_constraint_fail() {
        let errors = validate_constraints_src(
            r#"
type Tag = {_ String}

@assoc [Tag]
type Event = {...}

type QueryItem = {
    @constraint forall(tags, t => forall(events, e => e.assoc(t)))
    events []Event
    tags []Tag
}

type Container = { items []QueryItem }

tag1 = Tag {x String}
tag2 = Tag {y String}
ev1 = Event<tag1> {a String}
container = Container {
    items [
        { events [ev1], tags [tag1, tag2] }
    ]
}
"#,
        );
        assert!(!errors.is_empty());
        assert!(errors[0].message.contains("Constraint failed"));
    }

    #[test]
    fn test_nested_constraint_deep() {
        let errors = validate_constraints_src(
            r#"
type Inner = {
    @constraint x > 0
    x Int
}

type Outer = {
    nested {
        items []Inner
    }
}

outer = Outer {
    nested {
        items [
            {x 5},
            {x 10}
        ]
    }
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_nested_constraint_deep_fail() {
        let errors = validate_constraints_src(
            r#"
type Inner = {
    @constraint x > 0
    x Int
}

type Outer = {
    nested {
        items []Inner
    }
}

outer = Outer {
    nested {
        items [
            {x 5},
            {x -1}
        ]
    }
}
"#,
        );
        assert!(!errors.is_empty());
    }
}
