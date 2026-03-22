use crate::ast::*;
use crate::resolve::TypeEnv;
use crate::span::S;
use serde_json::{json, Map, Value as JsonValue};
use std::collections::HashSet;

// --- Public API ---

pub fn emit_schema(file: &File, env: &TypeEnv) -> JsonValue {
    let mut out = Map::new();
    for inst in file.instances() {
        if !is_main(inst) {
            continue;
        }
        out.insert(inst.name.node.clone(), emit_instance(inst, env));
    }
    JsonValue::Object(out)
}

pub struct EmitResult {
    /// The @main instance with @artifact binding refs replaced by { "$ref": "name.json" }
    pub main: JsonValue,
    /// Each @artifact instance: (name, fully-inlined JsonValue)
    pub artifacts: Vec<(String, JsonValue)>,
}

pub fn emit_schema_split(file: &File, env: &TypeEnv) -> EmitResult {
    let artifact_names: HashSet<&str> = env
        .instances
        .values()
        .filter(|i| is_artifact(&i.node))
        .map(|i| i.node.name.node.as_str())
        .collect();

    // Emit @main with $ref substitution for artifacts
    let mut main_map = Map::new();
    for inst in file.instances() {
        if !is_main(inst) {
            continue;
        }
        main_map.insert(
            inst.name.node.clone(),
            emit_value_split(&inst.body.node, Some(&inst.type_name.node), env, &artifact_names),
        );
    }

    // Emit each artifact fully inlined
    let mut artifacts: Vec<(String, JsonValue)> = env
        .instances
        .values()
        .filter(|i| is_artifact(&i.node))
        .map(|i| (i.node.name.node.clone(), emit_instance(&i.node, env)))
        .collect();
    artifacts.sort_by(|a, b| a.0.cmp(&b.0));

    EmitResult {
        main: JsonValue::Object(main_map),
        artifacts,
    }
}

// --- Instance emission ---

fn is_main(inst: &Instance) -> bool {
    inst.annotations
        .iter()
        .any(|a| matches!(a.node, Annotation::Main))
}

fn is_artifact(inst: &Instance) -> bool {
    inst.annotations
        .iter()
        .any(|a| matches!(a.node, Annotation::Artifact))
}

fn emit_instance(inst: &Instance, env: &TypeEnv) -> JsonValue {
    emit_value(&inst.body.node, Some(&inst.type_name.node), env)
}

// --- Value emission (original, always inlines) ---

fn emit_value(value: &Value, type_name: Option<&str>, env: &TypeEnv) -> JsonValue {
    match value {
        Value::TypeRef(name) => json!(name),
        Value::LitString(s) => json!(s),
        Value::LitInt(n) => json!(n),
        Value::LitBool(b) => json!(b),
        Value::BindingRef(name) => emit_binding_ref(name, env),
        Value::Struct(fields) => emit_struct(fields, type_name, env),
        Value::List(elements) => emit_list(elements, type_name, env),
        Value::Variant(name, body) => emit_variant(name, &body.node, env),
        Value::Refinement(base, _assocs, overrides) => emit_refinement(base, overrides, env),
    }
}

fn emit_binding_ref(name: &str, env: &TypeEnv) -> JsonValue {
    match env.get_instance(name) {
        Some(inst) => emit_instance(&inst.node, env),
        None => json!({ "$ref": name }),
    }
}

fn emit_struct(
    fields: &[S<InstanceField>],
    type_name: Option<&str>,
    env: &TypeEnv,
) -> JsonValue {
    let mut map = Map::new();
    if let Some(tn) = type_name {
        map.insert("$type".to_string(), json!(tn));
    }
    for f in fields {
        let field_name = &f.node.name.node;
        let field_type = type_name.and_then(|tn| resolve_field_type(tn, field_name, env));
        map.insert(
            field_name.clone(),
            emit_value(&f.node.value.node, field_type.as_deref(), env),
        );
    }
    JsonValue::Object(map)
}

fn emit_list(
    elements: &[S<ListElement>],
    elem_type: Option<&str>,
    env: &TypeEnv,
) -> JsonValue {
    JsonValue::Array(
        elements
            .iter()
            .map(|e| emit_list_element(&e.node, elem_type, env))
            .collect(),
    )
}

fn emit_list_element(elem: &ListElement, elem_type: Option<&str>, env: &TypeEnv) -> JsonValue {
    match elem {
        ListElement::Value(v) => emit_value(v, elem_type, env),
        ListElement::BindingRef(name) => emit_binding_ref(name, env),
        ListElement::Refinement(base, _assocs, overrides) => emit_refinement(base, overrides, env),
    }
}

fn emit_variant(name: &str, body: &Value, env: &TypeEnv) -> JsonValue {
    match body {
        Value::Struct(fields) if fields.is_empty() => json!(name),
        _ => emit_value(body, Some(name), env),
    }
}

fn emit_refinement(base: &str, overrides: &[S<InstanceField>], env: &TypeEnv) -> JsonValue {
    let base_type = env
        .get_instance(base)
        .map(|i| i.node.type_name.node.clone());
    let mut val = emit_binding_ref(base, env);
    if let JsonValue::Object(ref mut map) = val {
        for f in overrides {
            let field_name = &f.node.name.node;
            let field_type =
                base_type
                    .as_deref()
                    .and_then(|tn| resolve_field_type(tn, field_name, env));
            map.insert(
                field_name.clone(),
                emit_value(&f.node.value.node, field_type.as_deref(), env),
            );
        }
    }
    val
}

// --- Value emission (split mode: $ref for artifacts) ---

fn emit_value_split(
    value: &Value,
    type_name: Option<&str>,
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    match value {
        Value::TypeRef(name) => json!(name),
        Value::LitString(s) => json!(s),
        Value::LitInt(n) => json!(n),
        Value::LitBool(b) => json!(b),
        Value::BindingRef(name) => emit_binding_ref_split(name, env, artifacts),
        Value::Struct(fields) => emit_struct_split(fields, type_name, env, artifacts),
        Value::List(elements) => emit_list_split(elements, type_name, env, artifacts),
        Value::Variant(name, body) => emit_variant_split(name, &body.node, env, artifacts),
        Value::Refinement(base, _assocs, overrides) => {
            emit_refinement_split(base, overrides, env, artifacts)
        }
    }
}

fn emit_binding_ref_split(name: &str, env: &TypeEnv, artifacts: &HashSet<&str>) -> JsonValue {
    if artifacts.contains(name) {
        json!({ "$ref": format!("{}.json", name) })
    } else {
        emit_binding_ref(name, env)
    }
}

fn emit_struct_split(
    fields: &[S<InstanceField>],
    type_name: Option<&str>,
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    let mut map = Map::new();
    if let Some(tn) = type_name {
        map.insert("$type".to_string(), json!(tn));
    }
    for f in fields {
        let field_name = &f.node.name.node;
        let field_type = type_name.and_then(|tn| resolve_field_type(tn, field_name, env));
        map.insert(
            field_name.clone(),
            emit_value_split(&f.node.value.node, field_type.as_deref(), env, artifacts),
        );
    }
    JsonValue::Object(map)
}

fn emit_list_split(
    elements: &[S<ListElement>],
    elem_type: Option<&str>,
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    JsonValue::Array(
        elements
            .iter()
            .map(|e| emit_list_element_split(&e.node, elem_type, env, artifacts))
            .collect(),
    )
}

fn emit_list_element_split(
    elem: &ListElement,
    elem_type: Option<&str>,
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    match elem {
        ListElement::Value(v) => emit_value_split(v, elem_type, env, artifacts),
        ListElement::BindingRef(name) => emit_binding_ref_split(name, env, artifacts),
        ListElement::Refinement(base, _assocs, overrides) => {
            emit_refinement_split(base, overrides, env, artifacts)
        }
    }
}

fn emit_variant_split(
    name: &str,
    body: &Value,
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    match body {
        Value::Struct(fields) if fields.is_empty() => json!(name),
        _ => emit_value_split(body, Some(name), env, artifacts),
    }
}

fn emit_refinement_split(
    base: &str,
    overrides: &[S<InstanceField>],
    env: &TypeEnv,
    artifacts: &HashSet<&str>,
) -> JsonValue {
    let base_type = env
        .get_instance(base)
        .map(|i| i.node.type_name.node.clone());
    let mut val = emit_binding_ref_split(base, env, artifacts);
    if let JsonValue::Object(ref mut map) = val {
        for f in overrides {
            let field_name = &f.node.name.node;
            let field_type =
                base_type
                    .as_deref()
                    .and_then(|tn| resolve_field_type(tn, field_name, env));
            map.insert(
                field_name.clone(),
                emit_value_split(&f.node.value.node, field_type.as_deref(), env, artifacts),
            );
        }
    }
    val
}

// --- Type resolution ---

fn resolve_field_type(type_name: &str, field_name: &str, env: &TypeEnv) -> Option<String> {
    let decl = env.get_type(type_name)?;
    find_field_type(&decl.node.body.node, field_name, env)
}

fn find_field_type(ty: &TypeExpr, field_name: &str, env: &TypeEnv) -> Option<String> {
    match ty {
        TypeExpr::Named(name) => resolve_field_type(name, field_name, env),
        TypeExpr::Struct(kind) => {
            let fields = match kind {
                StructKind::Closed(f) | StructKind::Open(f) => f,
                StructKind::Anonymous(_) => return None,
            };
            fields
                .iter()
                .find(|f| f.node.name.node == field_name)
                .and_then(|f| extract_type_name(&f.node.ty.node, env))
        }
        TypeExpr::Intersection(left, right) => find_field_type(&left.node, field_name, env)
            .or_else(|| find_field_type(&right.node, field_name, env)),
        _ => None,
    }
}

fn extract_type_name(ty: &TypeExpr, env: &TypeEnv) -> Option<String> {
    match ty {
        TypeExpr::Named(name) => Some(name.clone()),
        TypeExpr::RefinableRef(name) => Some(name.clone()),
        TypeExpr::Reference(name) => Some(name.clone()),
        TypeExpr::List(_, inner) => extract_type_name(&inner.node, env),
        TypeExpr::Concrete(inner) => extract_type_name(&inner.node, env),
        _ => None,
    }
}
