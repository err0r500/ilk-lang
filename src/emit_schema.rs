use crate::ast::*;
use crate::resolve::TypeEnv;
use crate::span::S;
use serde_json::{json, Map, Value as JsonValue};

// --- Public API ---

pub fn emit_schema(file: &File, env: &TypeEnv) -> JsonValue {
    let mut out = Map::new();
    for inst in file.instances() {
        if !inst.is_main() {
            continue;
        }
        out.insert(inst.name.node.clone(), emit_instance(inst, env));
    }
    JsonValue::Object(out)
}

// --- Instance emission ---

fn emit_instance(inst: &Instance, env: &TypeEnv) -> JsonValue {
    emit_value(&inst.body.node, Some(&inst.type_name.node), env)
}

// --- Value emission ---

fn emit_value(value: &Value, type_name: Option<&str>, env: &TypeEnv) -> JsonValue {
    match value {
        Value::TypeRef(name) => json!(name),
        Value::ListType(_card, elem) => json!([emit_value(&elem.node, type_name, env)]),
        Value::LitString(s) => json!(s),
        Value::LitInt(n) => json!(n),
        Value::LitFloat(n) => json!(n),
        Value::LitBool(b) => json!(b),
        Value::BindingRef(name) => emit_binding_ref(name, env),
        Value::Struct(fields) => emit_struct(fields, type_name, env),
        Value::List(elements) => emit_list(elements, type_name, env),
        Value::Variant(name, body) => emit_variant(name, &body.node, env),
        Value::Refinement(base, overrides) => emit_refinement(base, overrides, env),
    }
}

fn emit_binding_ref(name: &str, env: &TypeEnv) -> JsonValue {
    match env.get_instance(name) {
        Some(inst) => emit_instance(&inst.node, env),
        None => json!({ "$ref": name }),
    }
}

fn emit_struct(fields: &[S<InstanceField>], type_name: Option<&str>, env: &TypeEnv) -> JsonValue {
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

fn emit_list(elements: &[S<ListElement>], elem_type: Option<&str>, env: &TypeEnv) -> JsonValue {
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
        ListElement::Refinement(base, overrides) => emit_refinement(base, overrides, env),
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
            let field_type = base_type
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

// --- Type resolution ---

pub(crate) fn resolve_field_type(
    type_name: &str,
    field_name: &str,
    env: &TypeEnv,
) -> Option<String> {
    let decl = env.get_meta(type_name)?;
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
                .and_then(|f| extract_type_name(&f.node.ty.node))
        }
        TypeExpr::Intersection(left, right) => find_field_type(&left.node, field_name, env)
            .or_else(|| find_field_type(&right.node, field_name, env)),
        _ => None,
    }
}

fn extract_type_name(ty: &TypeExpr) -> Option<String> {
    match ty {
        TypeExpr::Base(BaseType::Wildcard) => None,
        TypeExpr::Base(base) => Some(base.name().to_string()),
        TypeExpr::Named(name) => Some(name.clone()),
        TypeExpr::RefinableRef(name) => Some(name.clone()),
        TypeExpr::Reference(name) => Some(name.clone()),
        TypeExpr::List(_, inner) => extract_type_name(&inner.node),
        TypeExpr::Concrete(inner) => extract_type_name(&inner.node),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    fn schema_for(src: &str) -> JsonValue {
        let path = Path::new("test.ilk");
        let mut compiler = crate::Compiler::new();
        compiler.load(path, src).expect("load");
        compiler.validate(path).expect("validate");
        let file = compiler.get_file(path).unwrap();
        let env = compiler.get_env(path).unwrap();
        emit_schema(file, env)
    }

    #[test]
    fn snapshot_basic_struct() {
        insta::assert_json_snapshot!(schema_for(
            "meta T = {id! Uuid, name String, active Bool}\n@main\nm = T {id Uuid, name String, active Bool}\n"
        ));
    }

    #[test]
    fn snapshot_binding_refs_and_lists() {
        insta::assert_json_snapshot!(schema_for(
            "meta Item = {sku Concrete<String>, qty Int}\n\
             meta Cart = {items []Item, note Concrete<String>}\n\
             apple = Item {sku \"apple\", qty Int}\n\
             pear = Item {sku \"pear\", qty Int}\n\
             @main\ncart = Cart {items [apple, pear], note \"demo\"}\n"
        ));
    }

    #[test]
    fn snapshot_refinement_merges_overrides() {
        insta::assert_json_snapshot!(schema_for(
            "meta Event = {...} & {kind Concrete<String>, ts Int}\n\
             meta Board = {events []-Event}\n\
             base = Event {kind \"base\", ts Int}\n\
             @main\nboard = Board {events [base & {kind \"refined\"}]}\n"
        ));
    }

    #[test]
    fn snapshot_variant_and_typed_list() {
        insta::assert_json_snapshot!(schema_for(
            "meta Resp = {status Concrete<Int>}\n\
             meta Api = {tags []String, resp Resp}\n\
             @main\napi = Api {tags []String, resp {status 200}}\n"
        ));
    }
}
