use crate::ast::*;
use crate::resolve::TypeEnv;
use serde_json::{json, Map, Value as JsonValue};

pub fn emit_json(file: &File, env: &TypeEnv) -> JsonValue {
    let types = emit_types(file);
    let instances = emit_instances(file, env);

    json!({
        "types": types,
        "instances": instances
    })
}

fn emit_types(file: &File) -> JsonValue {
    let mut types = Map::new();

    for decl in file.type_decls() {
        let name = &decl.name.node;
        types.insert(name.clone(), emit_type_def(&decl.body.node));
    }

    JsonValue::Object(types)
}

fn emit_type_def(ty: &TypeExpr) -> JsonValue {
    match ty {
        TypeExpr::Base(base) => json!({
            "kind": "base",
            "type": format!("{:?}", base)
        }),
        TypeExpr::Named(name) => json!({
            "kind": "alias",
            "target": name
        }),
        TypeExpr::RefinableRef(name) => json!({
            "kind": "refinable_ref",
            "target": name
        }),
        TypeExpr::Reference(name) => json!({
            "kind": "reference",
            "target": name
        }),
        TypeExpr::Concrete(inner) => json!({
            "kind": "concrete",
            "inner": emit_type_def(&inner.node)
        }),
        TypeExpr::LitString(s) => json!({
            "kind": "literal",
            "value": s
        }),
        TypeExpr::LitInt(n) => json!({
            "kind": "literal",
            "value": n
        }),
        TypeExpr::LitBool(b) => json!({
            "kind": "literal",
            "value": b
        }),
        TypeExpr::Struct(kind) => emit_struct_type(kind),
        TypeExpr::List(card, inner) => json!({
            "kind": "list",
            "cardinality": emit_cardinality(card),
            "element": emit_type_def(&inner.node)
        }),
        TypeExpr::Union(variants) => {
            let vs: Vec<JsonValue> = variants.iter().map(|v| emit_type_def(&v.node)).collect();
            json!({
                "kind": "union",
                "variants": vs
            })
        }
        TypeExpr::Intersection(left, right) => json!({
            "kind": "intersection",
            "left": emit_type_def(&left.node),
            "right": emit_type_def(&right.node)
        }),
    }
}

fn emit_struct_type(kind: &StructKind) -> JsonValue {
    match kind {
        StructKind::Closed(fields) => {
            let mut field_map = Map::new();
            for f in fields {
                field_map.insert(
                    f.node.name.node.clone(),
                    json!({
                        "type": emit_type_def(&f.node.ty.node),
                        "optional": f.node.optional
                    }),
                );
            }
            json!({
                "kind": "struct",
                "open": false,
                "fields": JsonValue::Object(field_map)
            })
        }
        StructKind::Open(fields) => {
            let mut field_map = Map::new();
            for f in fields {
                field_map.insert(
                    f.node.name.node.clone(),
                    json!({
                        "type": emit_type_def(&f.node.ty.node),
                        "optional": f.node.optional
                    }),
                );
            }
            json!({
                "kind": "struct",
                "open": true,
                "fields": JsonValue::Object(field_map)
            })
        }
        StructKind::Anonymous(types) => {
            let ts: Vec<JsonValue> = types
                .iter()
                .map(|t| t.as_ref().map(|t| emit_type_def(&t.node)).unwrap_or(json!(null)))
                .collect();
            json!({
                "kind": "struct",
                "anonymous": true,
                "slots": ts
            })
        }
    }
}

fn emit_cardinality(card: &Cardinality) -> JsonValue {
    match card {
        Cardinality::Any => json!("[]"),
        Cardinality::Exact(n) => json!(format!("[{}]", n)),
        Cardinality::AtLeast(n) => json!(format!("[{}..]", n)),
        Cardinality::AtMost(n) => json!(format!("[..{}]", n)),
        Cardinality::Range(n, m) => json!(format!("[{}..{}]", n, m)),
    }
}

fn emit_instances(file: &File, env: &TypeEnv) -> JsonValue {
    let mut instances = Map::new();

    for inst in file.instances() {
        // Only emit @main instances
        if !inst.annotations.iter().any(|a| matches!(a.node, Annotation::Main)) {
            continue;
        }

        let name = &inst.name.node;
        let type_name = &inst.type_name.node;
        let mut obj = emit_value(&inst.body.node, env);

        if let JsonValue::Object(ref mut map) = obj {
            map.insert("$type".to_string(), json!(type_name));
        }

        instances.insert(name.clone(), obj);
    }

    JsonValue::Object(instances)
}

fn emit_value(value: &Value, env: &TypeEnv) -> JsonValue {
    match value {
        Value::TypeRef(name) => json!({ "$typeRef": name }),
        Value::LitString(s) => json!(s),
        Value::LitInt(n) => json!(n),
        Value::LitBool(b) => json!(b),
        Value::BindingRef(name) => {
            // Inline referenced instance
            if let Some(inst) = env.get_instance(name) {
                emit_value(&inst.node.body.node, env)
            } else {
                json!({ "$ref": name })
            }
        }
        Value::Struct(fields) => {
            let mut map = Map::new();
            for f in fields {
                let key = f.node.name.node.clone();
                let val = emit_value(&f.node.value.node, env);
                map.insert(key, val);
            }
            JsonValue::Object(map)
        }
        Value::List(elements) => {
            let items: Vec<JsonValue> = elements
                .iter()
                .map(|e| emit_list_element(&e.node, env))
                .collect();
            JsonValue::Array(items)
        }
        Value::Variant(name, body) => {
            match &body.node {
                Value::Struct(fields) if fields.is_empty() => {
                    // Unit variant
                    json!(name)
                }
                _ => {
                    json!({
                        "$variant": name,
                        "value": emit_value(&body.node, env)
                    })
                }
            }
        }
        Value::Refinement(base, _assocs, fields) => {
            // Resolve base and merge fields
            let mut base_val = if let Some(inst) = env.get_instance(base) {
                emit_value(&inst.node.body.node, env)
            } else {
                json!({ "$ref": base })
            };

            if let JsonValue::Object(ref mut map) = base_val {
                for f in fields {
                    let key = f.node.name.node.clone();
                    let val = emit_value(&f.node.value.node, env);
                    map.insert(key, val);
                }
            }
            base_val
        }
    }
}

fn emit_list_element(elem: &ListElement, env: &TypeEnv) -> JsonValue {
    match elem {
        ListElement::Value(v) => emit_value(v, env),
        ListElement::BindingRef(name) => {
            if let Some(inst) = env.get_instance(name) {
                emit_value(&inst.node.body.node, env)
            } else {
                json!({ "$ref": name })
            }
        }
        ListElement::Refinement(base, _assocs, fields) => {
            let mut base_val = if let Some(inst) = env.get_instance(base) {
                emit_value(&inst.node.body.node, env)
            } else {
                json!({ "$ref": base })
            };

            if let JsonValue::Object(ref mut map) = base_val {
                for f in fields {
                    let key = f.node.name.node.clone();
                    let val = emit_value(&f.node.value.node, env);
                    map.insert(key, val);
                }
            }
            base_val
        }
    }
}
