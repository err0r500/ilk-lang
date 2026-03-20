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

#[derive(Debug, Clone)]
struct FailureTrace {
    bindings: Vec<(String, String)>,
}

impl FailureTrace {
    fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    fn with_binding(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.bindings.push((name.into(), value.into()));
        self
    }

    fn merge(mut self, other: FailureTrace) -> Self {
        self.bindings.extend(other.bindings);
        self
    }
}

#[derive(Debug, Clone)]
enum ConstraintError {
    Eval(String),
    Failed(FailureTrace),
}

pub fn validate_constraints(ctx: &ValidationContext, inst: &Instance) -> Vec<Diagnostic> {
    let mut errors = Vec::new();
    let type_name = &inst.type_name.node;
    if let Some(type_decl) = ctx.env.get_type(type_name) {
        validate_instance_constraints(ctx, inst, &type_decl.node, &mut errors);
    }
    errors
}

fn validate_instance_constraints(
    ctx: &ValidationContext,
    inst: &Instance,
    type_decl: &TypeDecl,
    errors: &mut Vec<Diagnostic>,
) {
    let constraints: Vec<_> = find_constraints_in_type(&type_decl.body);
    let type_fields = type_field_names(&type_decl.body);

    for constraint in constraints {
        let env = build_eval_env(ctx, inst);
        let assocs = build_assoc_map(inst);
        let fail_msg = format!("Constraint failed for instance '{}'", inst.name.node);
        report_constraint_result(
            eval_constraint(&constraint.node, &env, &assocs, ctx),
            &constraint.span,
            &inst.name.span,
            &fail_msg,
            &type_fields,
            ctx,
            errors,
        );
    }

    // Recursively validate constraints on nested structures
    validate_nested_constraints(&inst.body, &type_decl.body, &inst.name.node, ctx, errors);
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
                let type_fields = type_field_names(&type_decl.node.body);
                if !constraints.is_empty() {
                    for (i, elem) in elements.iter().enumerate() {
                        if let ListElement::Value(Value::Struct(fields)) = &elem.node {
                            let env = build_env_from_fields(fields, ctx);
                            let assocs = HashSet::new();

                            for constraint in &constraints {
                                let fail_msg = format!(
                                    "Constraint failed for inline {} at {}[{}]",
                                    type_decl.node.name.node, parent_path, i
                                );
                                report_constraint_result(
                                    eval_constraint(&constraint.node, &env, &assocs, ctx),
                                    &elem.span,
                                    &elem.span,
                                    &fail_msg,
                                    &type_fields,
                                    ctx,
                                    errors,
                                );
                            }

                            let inline_value =
                                S::new(Value::Struct(fields.clone()), elem.span.clone());
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

        // Handle inline struct values with named types (e.g., endpoint HttpEndpoint with inline value)
        (Value::Struct(val_fields), TypeExpr::Named(type_name)) => {
            if let Some(type_decl) = ctx.env.get_type(type_name) {
                // Check constraints on this inline struct
                let constraints = find_constraints_in_type(&type_decl.node.body);
                let type_fields = type_field_names(&type_decl.node.body);
                if !constraints.is_empty() {
                    let env = build_env_from_fields(val_fields, ctx);
                    let assocs = HashSet::new();

                    for constraint in &constraints {
                        let fail_msg = format!(
                            "Constraint failed for inline {} at {}",
                            type_name, parent_path
                        );
                        report_constraint_result(
                            eval_constraint(&constraint.node, &env, &assocs, ctx),
                            &constraint.span,
                            &value.span,
                            &fail_msg,
                            &type_fields,
                            ctx,
                            errors,
                        );
                    }
                }

                // Recurse into nested fields
                validate_nested_constraints(value, &type_decl.node.body, parent_path, ctx, errors);
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

/// Extract field names declared in a type expression.
fn type_field_names(ty: &S<TypeExpr>) -> HashSet<String> {
    let mut names = HashSet::new();

    match &ty.node {
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            for field in fields {
                names.insert(field.node.name.node.clone());
            }
        }
        TypeExpr::Intersection(left, right) => {
            names.extend(type_field_names(left));
            names.extend(type_field_names(right));
        }
        _ => {}
    }

    names
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
    inst.assocs.iter().map(|a| a.node.clone()).collect()
}

/// Report the outcome of a single constraint evaluation, pushing diagnostics as needed.
/// `constraint_span` points to the constraint in the type definition.
/// `instance_span` points to the instance being validated.
/// `type_fields` contains field names declared in the type - if an unknown variable error
/// refers to a field not in this set, the error points to the constraint (type error).
fn report_constraint_result(
    result: Result<EvalValue, ConstraintError>,
    constraint_span: &crate::span::Span,
    instance_span: &crate::span::Span,
    fail_message: &str,
    type_fields: &HashSet<String>,
    ctx: &ValidationContext,
    errors: &mut Vec<Diagnostic>,
) {
    match result {
        Ok(EvalValue::Bool(true)) => {}
        Ok(EvalValue::Bool(false)) => {
            // Constraint failed due to instance data → instance error
            errors.push(Diagnostic::error(
                instance_span.clone(),
                fail_message,
                ctx.path,
            ));
        }
        Ok(_) => {
            errors.push(Diagnostic::error(
                constraint_span.clone(),
                "Constraint must evaluate to boolean",
                ctx.path,
            ));
        }
        Err(ConstraintError::Eval(msg)) => {
            // Check if this is an "Unknown variable" error for a field that exists in the type
            let span = if msg.starts_with("Unknown variable: ") {
                let var_name = &msg["Unknown variable: ".len()..];
                if type_fields.contains(var_name) {
                    // Field declared in type but missing in instance → instance error
                    instance_span
                } else {
                    // Field not declared in type → type/constraint error
                    constraint_span
                }
            } else {
                instance_span
            };
            errors.push(Diagnostic::error(
                span.clone(),
                format!("Constraint evaluation error: {}", msg),
                ctx.path,
            ));
        }
        Err(ConstraintError::Failed(trace)) => {
            // Constraint failed due to instance data → instance error
            let mut msg = fail_message.to_string();
            if !trace.bindings.is_empty() {
                msg.push_str("\n  where:");
                for (name, value) in &trace.bindings {
                    msg.push_str(&format!("\n    {} = {}", name, value));
                }
            }
            errors.push(Diagnostic::error(instance_span.clone(), msg, ctx.path));
        }
    }
}

/// Resolve the assoc set for an item in a collection iteration.
fn assocs_for_item(item: &EvalValue, ctx: &ValidationContext) -> HashSet<String> {
    if let EvalValue::BindingRef(name) = item {
        ctx.get_instance(name)
            .map(|inst| inst.assocs.iter().map(|a| a.node.clone()).collect())
            .unwrap_or_default()
    } else {
        HashSet::new()
    }
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
                    ListElement::Refinement(name, _, _) => EvalValue::BindingRef(name.clone()),
                })
                .collect();
            EvalValue::List(vals)
        }
        Value::Variant(_, body) => value_to_eval_value(body, ctx),
        Value::Refinement(name, _, _) => EvalValue::BindingRef(name.clone()),
    }
}

fn eval_constraint(
    expr: &ConstraintExpr,
    env: &HashMap<String, EvalValue>,
    assocs: &HashSet<String>,
    ctx: &ValidationContext,
) -> Result<EvalValue, ConstraintError> {
    match expr {
        ConstraintExpr::Bool(b) => Ok(EvalValue::Bool(*b)),
        ConstraintExpr::Int(n) => Ok(EvalValue::Int(*n)),

        ConstraintExpr::Var(name) => env
            .get(name)
            .cloned()
            .ok_or_else(|| ConstraintError::Eval(format!("Unknown variable: {}", name))),

        ConstraintExpr::FieldAccess(obj, field) => {
            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                EvalValue::Struct(map) => map
                    .get(field)
                    .cloned()
                    .ok_or_else(|| ConstraintError::Eval(format!("Unknown field: {}", field))),
                EvalValue::BindingRef(name) => {
                    if let Some(inst) = ctx.get_instance(&name) {
                        if let Value::Struct(fields) = &inst.body.node {
                            if let Some(f) = fields.iter().find(|f| &f.node.name.node == field) {
                                return Ok(value_to_eval_value(&f.node.value, ctx));
                            }
                        }
                    }
                    Err(ConstraintError::Eval(format!(
                        "Cannot access field {} on {}",
                        field, name
                    )))
                }
                _ => Err(ConstraintError::Eval(format!(
                    "Cannot access field {} on non-struct",
                    field
                ))),
            }
        }

        ConstraintExpr::All(col_expr, var, body) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            match col_val {
                EvalValue::List(ref items) => {
                    for item in items {
                        let mut inner_env = env.clone();
                        inner_env.insert(var.clone(), item.clone());
                        let item_assocs = assocs_for_item(item, ctx);
                        match eval_constraint(&body.node, &inner_env, &item_assocs, ctx) {
                            Ok(EvalValue::Bool(false)) => {
                                let trace = FailureTrace::new()
                                    .with_binding(var, eval_value_to_string(item));
                                return Err(ConstraintError::Failed(trace));
                            }
                            Ok(EvalValue::Bool(true)) => continue,
                            Ok(_) => {
                                return Err(ConstraintError::Eval(
                                    "all body must be boolean".to_string(),
                                ))
                            }
                            Err(ConstraintError::Failed(inner_trace)) => {
                                let trace = FailureTrace::new()
                                    .with_binding(var, eval_value_to_string(item))
                                    .merge(inner_trace);
                                return Err(ConstraintError::Failed(trace));
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(EvalValue::Bool(true))
                }
                EvalValue::Set(ref items) => {
                    let col_str = eval_value_to_string(&col_val);
                    for item in items {
                        let mut inner_env = env.clone();
                        inner_env.insert(var.clone(), EvalValue::String(item.clone()));

                        match eval_constraint(&body.node, &inner_env, assocs, ctx) {
                            Ok(EvalValue::Bool(false)) => {
                                let trace =
                                    FailureTrace::new().with_binding(var, format!("\"{}\"", item));
                                return Err(ConstraintError::Failed(trace));
                            }
                            Ok(EvalValue::Bool(true)) => continue,
                            Ok(_) => {
                                return Err(ConstraintError::Eval(
                                    "all body must be boolean".to_string(),
                                ))
                            }
                            Err(ConstraintError::Failed(inner_trace)) => {
                                let trace = FailureTrace::new()
                                    .with_binding(
                                        format!("iterating over {}", col_str),
                                        format!("failed at \"{}\"", item),
                                    )
                                    .merge(inner_trace);
                                return Err(ConstraintError::Failed(trace));
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Ok(EvalValue::Bool(true))
                }
                _ => Err(ConstraintError::Eval(
                    "all collection must be a list or set".to_string(),
                )),
            }
        }

        ConstraintExpr::Exists(col_expr, var, body) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            if let EvalValue::List(items) = col_val {
                for item in &items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());
                    let item_assocs = assocs_for_item(item, ctx);
                    let result = eval_constraint(&body.node, &inner_env, &item_assocs, ctx)?;
                    if let EvalValue::Bool(true) = result {
                        return Ok(EvalValue::Bool(true));
                    }
                }
                Ok(EvalValue::Bool(false))
            } else {
                Err(ConstraintError::Eval(
                    "exists collection must be a list".to_string(),
                ))
            }
        }

        ConstraintExpr::Unique(col_expr, var, body) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            if let EvalValue::List(items) = col_val {
                let mut seen = HashSet::new();
                for item in &items {
                    let mut inner_env = env.clone();
                    inner_env.insert(var.clone(), item.clone());

                    let result = eval_constraint(&body.node, &inner_env, assocs, ctx)?;
                    let key = eval_value_to_string(&result);
                    if seen.contains(&key) {
                        let trace = FailureTrace::new().with_binding("duplicate value", key);
                        return Err(ConstraintError::Failed(trace));
                    }
                    seen.insert(key);
                }
                Ok(EvalValue::Bool(true))
            } else {
                Err(ConstraintError::Eval(
                    "unique collection must be a list".to_string(),
                ))
            }
        }

        ConstraintExpr::Count(col_expr) => {
            let col_val = eval_constraint(&col_expr.node, env, assocs, ctx)?;

            if let EvalValue::List(items) = col_val {
                Ok(EvalValue::Int(items.len() as i64))
            } else {
                Err(ConstraintError::Eval(
                    "count argument must be a list".to_string(),
                ))
            }
        }

        ConstraintExpr::Assoc(obj, tag) => {
            let tag_val = eval_constraint(&tag.node, env, assocs, ctx)?;
            let tag_name = match tag_val {
                EvalValue::BindingRef(name) => name,
                _ => {
                    return Err(ConstraintError::Eval(
                        "assoc tag must be a binding reference".to_string(),
                    ))
                }
            };

            let obj_val = eval_constraint(&obj.node, env, assocs, ctx)?;
            match obj_val {
                EvalValue::BindingRef(name) => {
                    if let Some(inst) = ctx.get_instance(&name) {
                        let has = inst.assocs.iter().any(|a| a.node == tag_name);
                        Ok(EvalValue::Bool(has))
                    } else {
                        Err(ConstraintError::Eval(format!("Unknown instance: {}", name)))
                    }
                }
                _ => Ok(EvalValue::Bool(assocs.contains(&tag_name))),
            }
        }

        ConstraintExpr::TemplateVars(expr) => {
            let val = eval_constraint(&expr.node, env, assocs, ctx)?;
            if let EvalValue::String(s) = val {
                let vars = extract_template_vars(&s);
                Ok(EvalValue::Set(vars))
            } else {
                Err(ConstraintError::Eval(
                    "templateVars requires a string".to_string(),
                ))
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
                    Err(ConstraintError::Eval(format!(
                        "Cannot get keys of {}",
                        name
                    )))
                }
                _ => Err(ConstraintError::Eval("keys requires a struct".to_string())),
            }
        }

        ConstraintExpr::And(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            match l {
                EvalValue::Bool(false) => Ok(EvalValue::Bool(false)),
                EvalValue::Bool(true) => {
                    let r = eval_constraint(&right.node, env, assocs, ctx)?;
                    match r {
                        EvalValue::Bool(b) => Ok(EvalValue::Bool(b)),
                        _ => Err(ConstraintError::Eval(
                            "&& requires boolean operands".to_string(),
                        )),
                    }
                }
                _ => Err(ConstraintError::Eval(
                    "&& requires boolean operands".to_string(),
                )),
            }
        }

        ConstraintExpr::Or(left, right) => {
            let l = eval_constraint(&left.node, env, assocs, ctx)?;
            match l {
                EvalValue::Bool(true) => Ok(EvalValue::Bool(true)),
                EvalValue::Bool(false) => {
                    let r = eval_constraint(&right.node, env, assocs, ctx)?;
                    match r {
                        EvalValue::Bool(b) => Ok(EvalValue::Bool(b)),
                        _ => Err(ConstraintError::Eval(
                            "|| requires boolean operands".to_string(),
                        )),
                    }
                }
                _ => Err(ConstraintError::Eval(
                    "|| requires boolean operands".to_string(),
                )),
            }
        }

        ConstraintExpr::Not(inner) => {
            let v = eval_constraint(&inner.node, env, assocs, ctx)?;
            match v {
                EvalValue::Bool(b) => Ok(EvalValue::Bool(!b)),
                _ => Err(ConstraintError::Eval(
                    "! requires boolean operand".to_string(),
                )),
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

            match (&e, &s) {
                (EvalValue::String(key), EvalValue::Set(set_vals)) => {
                    if set_vals.contains(key) {
                        Ok(EvalValue::Bool(true))
                    } else {
                        let trace = FailureTrace::new().with_binding(
                            format!("\"{}\" in {}", key, eval_value_to_string(&s)),
                            "false",
                        );
                        Err(ConstraintError::Failed(trace))
                    }
                }
                (EvalValue::BindingRef(key), EvalValue::Set(set_vals)) => {
                    if set_vals.contains(key) {
                        Ok(EvalValue::Bool(true))
                    } else {
                        let trace = FailureTrace::new().with_binding(
                            format!("\"{}\" in {}", key, eval_value_to_string(&s)),
                            "false",
                        );
                        Err(ConstraintError::Failed(trace))
                    }
                }
                (EvalValue::BindingRef(key), EvalValue::List(items)) => {
                    let found = items
                        .iter()
                        .any(|item| matches!(item, EvalValue::BindingRef(k) if k == key));
                    if found {
                        Ok(EvalValue::Bool(true))
                    } else {
                        let trace = FailureTrace::new().with_binding(
                            format!("{} in {}", key, eval_value_to_string(&s)),
                            "false",
                        );
                        Err(ConstraintError::Failed(trace))
                    }
                }
                _ => Err(ConstraintError::Eval(
                    "in requires element and set or list".to_string(),
                )),
            }
        }

        ConstraintExpr::IsType(inner, type_name) => {
            let val = eval_constraint(&inner.node, env, assocs, ctx)?;
            let kind = type_kind(type_name, ctx);
            let matches = match kind {
                "list" => matches!(val, EvalValue::List(_)),
                "struct" => matches!(val, EvalValue::Struct(_) | EvalValue::BindingRef(_)),
                "bool" => matches!(val, EvalValue::Bool(_)),
                "int" => matches!(val, EvalValue::Int(_)),
                "string" => matches!(val, EvalValue::String(_)),
                _ => {
                    return Err(ConstraintError::Eval(format!(
                        "isType: unknown type '{}'",
                        type_name
                    )))
                }
            };
            Ok(EvalValue::Bool(matches))
        }

        ConstraintExpr::Lt(left, right) => {
            eval_int_cmp(left, right, "<", env, assocs, ctx, |a, b| a < b)
        }
        ConstraintExpr::Le(left, right) => {
            eval_int_cmp(left, right, "<=", env, assocs, ctx, |a, b| a <= b)
        }
        ConstraintExpr::Gt(left, right) => {
            eval_int_cmp(left, right, ">", env, assocs, ctx, |a, b| a > b)
        }
        ConstraintExpr::Ge(left, right) => {
            eval_int_cmp(left, right, ">=", env, assocs, ctx, |a, b| a >= b)
        }
    }
}

fn eval_int_cmp(
    left: &S<ConstraintExpr>,
    right: &S<ConstraintExpr>,
    op: &str,
    env: &HashMap<String, EvalValue>,
    assocs: &HashSet<String>,
    ctx: &ValidationContext,
    cmp: impl Fn(i64, i64) -> bool,
) -> Result<EvalValue, ConstraintError> {
    let l = eval_constraint(&left.node, env, assocs, ctx)?;
    let r = eval_constraint(&right.node, env, assocs, ctx)?;
    match (&l, &r) {
        (EvalValue::Int(a), EvalValue::Int(b)) => {
            if cmp(*a, *b) {
                Ok(EvalValue::Bool(true))
            } else {
                let trace =
                    FailureTrace::new().with_binding(format!("{} {} {}", a, op, b), "false");
                Err(ConstraintError::Failed(trace))
            }
        }
        _ => Err(ConstraintError::Eval(format!(
            "{} requires integer operands",
            op
        ))),
    }
}

fn type_kind(type_name: &str, ctx: &ValidationContext) -> &'static str {
    match ctx.env.get_type(type_name) {
        Some(decl) => match &decl.node.body.node {
            TypeExpr::List(_, _) => "list",
            TypeExpr::Struct(_) | TypeExpr::Intersection(_, _) => "struct",
            TypeExpr::Base(BaseType::Bool) | TypeExpr::LitBool(_) => "bool",
            TypeExpr::Base(BaseType::Int) | TypeExpr::LitInt(_) => "int",
            TypeExpr::Base(BaseType::String) | TypeExpr::LitString(_) => "string",
            TypeExpr::Named(inner) => type_kind(inner, ctx),
            _ => "unknown",
        },
        None => "unknown",
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
        EvalValue::Set(items) => format!(
            "{{{}}}",
            items.iter().cloned().collect::<Vec<_>>().join(", ")
        ),
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
        for inst in file.instances() {
            errors.extend(validate_constraints(&ctx, inst));
        }
        errors
    }

    #[test]
    fn test_all_true() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint all(items, i => true)
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
    fn test_all_empty() {
        let errors = validate_constraints_src(
            r#"
type Bar = {x Int}
type Foo = {
  @constraint all(items, i => true)
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
    @constraint all(tags, t => all(events, e => e.assoc(t)))
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
    @constraint all(tags, t => all(events, e => e.assoc(t)))
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

    #[test]
    fn test_constraint_error_missing_instance_field() {
        // Field in type, missing in instance → error at instance
        let src = "type Foo = {\n    @constraint x > 0\n    x Int\n}\nfoo = Foo {}";
        //         ^0          ^12            ^30      ^43 ^45^47
        // "foo" instance name starts at byte 45 (after "}\n")
        // Type definition ends at byte 44 ("}")
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        for inst in file.instances() {
            errors.extend(validate_constraints(&ctx, inst));
        }

        assert_eq!(errors.len(), 1);
        // Error should point to instance name (after type def ends at 44)
        assert!(
            errors[0].span.start >= 45,
            "Expected instance error span, got {:?}",
            errors[0].span
        );
    }

    #[test]
    fn test_constraint_error_undeclared_type_field() {
        // Field not in type → error at constraint
        let src = "type Foo = {\n    @constraint y > 0\n    x Int\n}\nfoo = Foo { x 1 }";
        //         ^0          ^12            ^30      ^43  ^48
        // @constraint starts at byte 16
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        for inst in file.instances() {
            errors.extend(validate_constraints(&ctx, inst));
        }

        assert_eq!(errors.len(), 1);
        // Error should point to constraint (within type def, before byte 48)
        assert!(
            errors[0].span.start < 48,
            "Expected constraint error span, got {:?}",
            errors[0].span
        );
    }

    #[test]
    fn test_constraint_failure_points_to_instance() {
        // Constraint evaluates to false → error at instance
        let src = "type Foo = {\n    @constraint x > 10\n    x Int\n}\nfoo = Foo { x 5 }";
        //         ^0          ^12             ^31      ^44 ^46
        let file = parse(src, Path::new("test.ilk")).unwrap();
        let env = resolve(&file, Path::new("test.ilk")).unwrap();
        let ctx = ValidationContext::new(&env, Path::new("test.ilk"));
        let mut errors = Vec::new();
        for inst in file.instances() {
            errors.extend(validate_constraints(&ctx, inst));
        }

        assert_eq!(errors.len(), 1);
        // Error should point to instance (after type def)
        assert!(
            errors[0].span.start >= 46,
            "Expected instance error span, got {:?}",
            errors[0].span
        );
    }

    #[test]
    fn test_is_type_struct() {
        let errors = validate_constraints_src(
            r#"
type Inner = {x Int}
type Foo = {
    @constraint isType(val, Inner)
    val Inner
}
inner = Inner {x Int}
foo = Foo {val inner}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_is_type_list() {
        let errors = validate_constraints_src(
            r#"
type Item = {x Int}
type Items = []Item
type Foo = {
    @constraint isType(val, Items)
    val []Item
}
i1 = Item {x Int}
foo = Foo {val [i1]}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_is_type_mismatch() {
        let errors = validate_constraints_src(
            r#"
type Resp = {status Int}
type Foo = {
    @constraint isType(val, Resp)
    val []Resp
}
foo = Foo {val []}
"#,
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_conditional_constraint_skip_for_error_variant() {
        // when then is a struct (error), the events constraint is vacuously skipped
        let errors = validate_constraints_src(
            r#"
type Event = {...}
type Resp = {status Int}
type Emitter = {
    @constraint isType(then, Resp) || all(then, e => e in emits)
    emits []Event
    then []Event | Resp
}
ev1 = Event {a String}
ok = Emitter {
    emits [ev1]
    then { status 404 }
}
"#,
        );
        assert!(errors.is_empty(), "{:?}", errors);
    }

    #[test]
    fn test_conditional_constraint_enforced_for_event_list() {
        // when then is a list, the events constraint is enforced
        let errors = validate_constraints_src(
            r#"
type Event = {...}
type Resp = {status Int}
type Emitter = {
    @constraint isType(then, Resp) || all(then, e => e in emits)
    emits []Event
    then []Event | Resp
}
ev1 = Event {a String}
ev2 = Event {b String}
bad = Emitter {
    emits [ev1]
    then [ev2]
}
"#,
        );
        assert!(!errors.is_empty());
    }
}
