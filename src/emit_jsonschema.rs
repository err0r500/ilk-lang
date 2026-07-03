//! Emit a valid JSON Schema (draft 2020-12) describing the `@main` instances.
//!
//! Unlike [`crate::emit_schema`] — which prints a shape document whose leaves are
//! bare type names — this walks the same instance tree but produces JSON Schema
//! fragments: base-type leaves become `{"type":…}`, instance-fixed literals become
//! `{"const":…}`, structs become `{"type":"object",…}`, and so on.

use crate::ast::*;
use crate::resolve::TypeEnv;
use crate::span::S;
use serde_json::{json, Map, Value as JsonValue};

const SCHEMA_URI: &str = "https://json-schema.org/draft/2020-12/schema";

/// Walking context: the type environment plus the `$defs` accumulated for any
/// named-type `$ref`s encountered during traversal.
struct Ctx<'a> {
    env: &'a TypeEnv,
    defs: Map<String, JsonValue>,
}

// --- Public API ---

pub fn emit_json_schema(file: &File, env: &TypeEnv) -> JsonValue {
    let mut ctx = Ctx {
        env,
        defs: Map::new(),
    };

    let mains: Vec<&Instance> = file.instances().filter(|i| is_main(i)).collect();

    let mut out = match mains.as_slice() {
        // Single @main → its schema sits at the document root.
        [inst] => match ctx.instance_schema(inst) {
            JsonValue::Object(map) => map,
            other => {
                let mut m = Map::new();
                m.insert("$ref".to_string(), other);
                m
            }
        },
        // Zero or many → a bag of named definitions keyed by instance name.
        _ => {
            for inst in &mains {
                let schema = ctx.instance_schema(inst);
                ctx.defs.insert(inst.name.node.clone(), schema);
            }
            Map::new()
        }
    };

    let mut doc = Map::new();
    doc.insert("$schema".to_string(), json!(SCHEMA_URI));
    doc.append(&mut out);
    if !ctx.defs.is_empty() {
        doc.insert("$defs".to_string(), JsonValue::Object(ctx.defs));
    }
    JsonValue::Object(doc)
}

fn is_main(inst: &Instance) -> bool {
    inst.annotations
        .iter()
        .any(|a| matches!(a.node, Annotation::Main))
}

// --- Instance / value emission ---

impl<'a> Ctx<'a> {
    fn instance_schema(&mut self, inst: &Instance) -> JsonValue {
        self.value_schema(&inst.body.node, Some(&inst.type_name.node))
    }

    fn value_schema(&mut self, value: &Value, type_name: Option<&str>) -> JsonValue {
        match value {
            Value::TypeRef(name) => self.type_ref_schema(name),
            Value::LitString(s) => json!({ "const": s }),
            Value::LitInt(n) => json!({ "const": n }),
            Value::LitBool(b) => json!({ "const": b }),
            Value::BindingRef(name) => self.binding_ref_schema(name),
            Value::Struct(fields) => self.struct_schema(fields, type_name),
            Value::List(elements) => {
                let items: Vec<JsonValue> = elements
                    .iter()
                    .map(|e| self.list_element_schema(&e.node, type_name))
                    .collect();
                array_schema(merge_items(items), None)
            }
            Value::ListType(card, elem) => {
                let item = self.value_schema(&elem.node, type_name);
                array_schema(Some(item), Some(card))
            }
            Value::Variant(name, body) => self.variant_schema(name, &body.node),
            Value::Refinement(base, overrides) => self.refinement_schema(base, overrides),
        }
    }

    /// A bare type name in value position: a base type, an identifier variant
    /// (fixed tag), or a named meta we lower into `$defs` and reference.
    fn type_ref_schema(&mut self, name: &str) -> JsonValue {
        if let Some(bt) = BaseType::from_name(name) {
            return base_type_schema(&bt);
        }
        if self.is_nullary_variant(name) {
            return json!({ "const": name });
        }
        self.named_ref(name)
    }

    fn binding_ref_schema(&mut self, name: &str) -> JsonValue {
        if let Some(inst) = self.env.get_instance(name) {
            return self.instance_schema(&inst.node.clone());
        }
        // An identifier-only union variant (e.g. `method POST`) is a fixed tag.
        if self.is_nullary_variant(name) {
            return json!({ "const": name });
        }
        // Otherwise a bare meta name in value position references a named type →
        // lower it into `$defs`.
        if self.env.get_meta(name).is_some() {
            return self.named_ref(name);
        }
        json!({ "$ref": format!("#/$defs/{name}") })
    }

    /// True when `name` denotes an identifier-only union variant — registered by
    /// resolve as a meta whose body is an empty closed struct (`src/resolve.rs`).
    fn is_nullary_variant(&self, name: &str) -> bool {
        self.env.get_meta(name).is_some_and(|decl| {
            matches!(&decl.node.body.node, TypeExpr::Struct(StructKind::Closed(f)) if f.is_empty())
        })
    }

    fn struct_schema(&mut self, fields: &[S<InstanceField>], type_name: Option<&str>) -> JsonValue {
        let mut props = Map::new();
        let mut required: Vec<JsonValue> = Vec::new();

        for f in fields {
            let field_name = &f.node.name.node;
            let field_type = type_name
                .and_then(|tn| crate::emit_schema::resolve_field_type(tn, field_name, self.env));
            let mut field_schema = self.value_schema(&f.node.value.node, field_type.as_deref());
            annotate_origin(&mut field_schema, &f.node.origin);
            props.insert(field_name.clone(), field_schema);
            if !f.node.optional {
                required.push(json!(field_name));
            }
        }

        let mut map = Map::new();
        map.insert("type".to_string(), json!("object"));
        if let Some(tn) = type_name {
            map.insert("_type".to_string(), json!(tn));
        }
        map.insert("properties".to_string(), JsonValue::Object(props));
        if !required.is_empty() {
            map.insert("required".to_string(), JsonValue::Array(required));
        }
        // `additionalProperties` follows the declared struct's openness when known.
        if let Some(open) = type_name.and_then(|tn| self.struct_is_open(tn)) {
            map.insert("additionalProperties".to_string(), json!(open));
        }
        JsonValue::Object(map)
    }

    fn list_element_schema(&mut self, elem: &ListElement, elem_type: Option<&str>) -> JsonValue {
        match elem {
            ListElement::Value(v) => self.value_schema(v, elem_type),
            ListElement::BindingRef(name) => self.binding_ref_schema(name),
            ListElement::Refinement(base, overrides) => self.refinement_schema(base, overrides),
        }
    }

    fn variant_schema(&mut self, name: &str, body: &Value) -> JsonValue {
        match body {
            // Unit variant → a fixed tag.
            Value::Struct(fields) if fields.is_empty() => json!({ "const": name }),
            _ => {
                let mut schema = self.value_schema(body, Some(name));
                if let JsonValue::Object(ref mut map) = schema {
                    map.insert("_type".to_string(), json!(name));
                }
                schema
            }
        }
    }

    fn refinement_schema(&mut self, base: &str, overrides: &[S<InstanceField>]) -> JsonValue {
        let base_type = self
            .env
            .get_instance(base)
            .map(|i| i.node.type_name.node.clone());
        let mut schema = self.binding_ref_schema(base);

        if let JsonValue::Object(ref mut map) = schema {
            let props = map
                .entry("properties")
                .or_insert_with(|| JsonValue::Object(Map::new()));
            if let JsonValue::Object(props) = props {
                for f in overrides {
                    let field_name = &f.node.name.node;
                    let field_type = base_type.as_deref().and_then(|tn| {
                        crate::emit_schema::resolve_field_type(tn, field_name, self.env)
                    });
                    let mut field_schema =
                        self.value_schema(&f.node.value.node, field_type.as_deref());
                    annotate_origin(&mut field_schema, &f.node.origin);
                    props.insert(field_name.clone(), field_schema);
                }
            }
        }
        schema
    }

    /// Register a named meta in `$defs` (if not already present) and return a
    /// `$ref` to it.
    fn named_ref(&mut self, name: &str) -> JsonValue {
        if !self.defs.contains_key(name) {
            // Insert a placeholder first to break cycles, then fill it in.
            self.defs.insert(name.to_string(), JsonValue::Bool(true));
            if let Some(decl) = self.env.get_meta(name) {
                let schema = self.type_expr_schema(&decl.node.body.node.clone());
                self.defs.insert(name.to_string(), schema);
            }
        }
        json!({ "$ref": format!("#/$defs/{name}") })
    }

    /// Resolve whether the named type ultimately denotes an open struct.
    fn struct_is_open(&self, type_name: &str) -> Option<bool> {
        let decl = self.env.get_meta(type_name)?;
        self.type_expr_is_open(&decl.node.body.node)
    }

    fn type_expr_is_open(&self, ty: &TypeExpr) -> Option<bool> {
        match ty {
            TypeExpr::Struct(StructKind::Open(_)) => Some(true),
            TypeExpr::Struct(StructKind::Closed(_)) => Some(false),
            TypeExpr::Named(name) => self.struct_is_open(name),
            TypeExpr::Concrete(inner) => self.type_expr_is_open(&inner.node),
            _ => None,
        }
    }

    // --- TypeExpr → schema (for named definitions) ---

    fn type_expr_schema(&mut self, ty: &TypeExpr) -> JsonValue {
        match ty {
            TypeExpr::Base(base) => base_type_schema(base),
            TypeExpr::Named(name) => self.named_ref(name),
            TypeExpr::RefinableRef(name) | TypeExpr::Reference(name) => self.named_ref(name),
            TypeExpr::Concrete(inner) => self.type_expr_schema(&inner.node),
            TypeExpr::LitString(s) => json!({ "const": s }),
            TypeExpr::LitInt(n) => json!({ "const": n }),
            TypeExpr::LitBool(b) => json!({ "const": b }),
            TypeExpr::Struct(kind) => self.struct_type_schema(kind),
            TypeExpr::List(card, inner) => {
                let item = self.type_expr_schema(&inner.node);
                array_schema(Some(item), Some(card))
            }
            TypeExpr::Union(variants) => self.union_schema(variants),
            TypeExpr::Intersection(left, right) => {
                json!({
                    "allOf": [
                        self.type_expr_schema(&left.node),
                        self.type_expr_schema(&right.node),
                    ]
                })
            }
        }
    }

    fn struct_type_schema(&mut self, kind: &StructKind) -> JsonValue {
        match kind {
            StructKind::Closed(fields) | StructKind::Open(fields) => {
                let open = matches!(kind, StructKind::Open(_));
                let mut props = Map::new();
                let mut required: Vec<JsonValue> = Vec::new();
                for f in fields {
                    let name = &f.node.name.node;
                    props.insert(name.clone(), self.type_expr_schema(&f.node.ty.node));
                    if !f.node.optional {
                        required.push(json!(name));
                    }
                }
                let mut map = Map::new();
                map.insert("type".to_string(), json!("object"));
                map.insert("properties".to_string(), JsonValue::Object(props));
                if !required.is_empty() {
                    map.insert("required".to_string(), JsonValue::Array(required));
                }
                map.insert("additionalProperties".to_string(), json!(open));
                JsonValue::Object(map)
            }
            // `{_}`, `{_ String}`, … — positional slots, modelled as a tuple.
            StructKind::Anonymous(slots) => {
                let prefix: Vec<JsonValue> = slots
                    .iter()
                    .map(|s| match s {
                        Some(t) => self.type_expr_schema(&t.node),
                        None => JsonValue::Bool(true),
                    })
                    .collect();
                json!({ "type": "array", "prefixItems": prefix })
            }
        }
    }

    fn union_schema(&mut self, variants: &[S<TypeExpr>]) -> JsonValue {
        // All-literal unions collapse to a JSON Schema `enum`.
        let consts: Option<Vec<JsonValue>> =
            variants.iter().map(|v| literal_const(&v.node)).collect();
        if let Some(values) = consts {
            return json!({ "enum": values });
        }
        let one_of: Vec<JsonValue> = variants
            .iter()
            .map(|v| self.type_expr_schema(&v.node))
            .collect();
        json!({ "oneOf": one_of })
    }
}

// --- Free helpers ---

/// Attach origin-derived JSON Schema annotations (`x-generated`, …) to a
/// field's property schema. No-op for origins without an annotation.
fn annotate_origin(schema: &mut JsonValue, origin: &FieldOrigin) {
    if let JsonValue::Object(map) = schema {
        if matches!(origin, FieldOrigin::Generated) {
            map.insert("x-generated".to_string(), json!(true));
        }
        // Future: FieldOrigin::Computed(paths) => "x-computed-from".
    }
}

fn array_schema(item: Option<JsonValue>, card: Option<&Cardinality>) -> JsonValue {
    let mut map = Map::new();
    map.insert("type".to_string(), json!("array"));
    if let Some(item) = item {
        map.insert("items".to_string(), item);
    }
    if let Some(card) = card {
        let (min, max) = cardinality_bounds(card);
        if let Some(min) = min {
            map.insert("minItems".to_string(), json!(min));
        }
        if let Some(max) = max {
            map.insert("maxItems".to_string(), json!(max));
        }
    }
    JsonValue::Object(map)
}

/// Collapse a list's element schemas into a single `items` schema: identical
/// elements share one schema; differing elements become `{"oneOf": […distinct…]}`
/// (first-seen order, deduplicated). Empty → no `items`.
fn merge_items(elements: Vec<JsonValue>) -> Option<JsonValue> {
    let mut distinct: Vec<JsonValue> = Vec::new();
    for e in elements {
        if !distinct.contains(&e) {
            distinct.push(e);
        }
    }
    match distinct.len() {
        0 => None,
        1 => Some(distinct.into_iter().next().unwrap()),
        _ => Some(json!({ "oneOf": distinct })),
    }
}

fn cardinality_bounds(card: &Cardinality) -> (Option<usize>, Option<usize>) {
    match card {
        Cardinality::Any => (None, None),
        Cardinality::Exact(n) => (Some(*n), Some(*n)),
        Cardinality::AtLeast(n) => (Some(*n), None),
        Cardinality::AtMost(m) => (None, Some(*m)),
        Cardinality::Range(n, m) => (Some(*n), Some(*m)),
    }
}

fn literal_const(ty: &TypeExpr) -> Option<JsonValue> {
    match ty {
        TypeExpr::LitString(s) => Some(json!(s)),
        TypeExpr::LitInt(n) => Some(json!(n)),
        TypeExpr::LitBool(b) => Some(json!(b)),
        _ => None,
    }
}

fn base_type_schema(base: &BaseType) -> JsonValue {
    match base {
        BaseType::String => json!({ "type": "string" }),
        BaseType::Int => json!({ "type": "integer" }),
        BaseType::Float => json!({ "type": "number" }),
        BaseType::Bool => json!({ "type": "boolean" }),
        BaseType::Uuid => json!({ "type": "string", "format": "uuid" }),
        BaseType::Date => json!({ "type": "string", "format": "date" }),
        BaseType::Timestamp => json!({ "type": "string", "format": "date-time" }),
        // No standard JSON Schema format for money; best-effort numeric.
        BaseType::Money => json!({ "type": "number" }),
        BaseType::Wildcard => json!({}),
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
        emit_json_schema(file, env)
    }

    #[test]
    fn base_type_leaves_and_root_header() {
        let s = schema_for(
            "meta T = {id Uuid, n String, c Int, f Float, b Bool}\n@main\nm = T {id Uuid, n String, c Int, f Float, b Bool}\n",
        );
        assert_eq!(s["$schema"], json!(SCHEMA_URI));
        assert_eq!(s["type"], json!("object"));
        assert_eq!(
            s["properties"]["id"],
            json!({"type":"string","format":"uuid"})
        );
        assert_eq!(s["properties"]["n"], json!({"type":"string"}));
        assert_eq!(s["properties"]["c"], json!({"type":"integer"}));
        assert_eq!(s["properties"]["f"], json!({"type":"number"}));
        assert_eq!(s["properties"]["b"], json!({"type":"boolean"}));
    }

    #[test]
    fn fixed_literals_become_const() {
        let s = schema_for(
            "meta T = {m \"GET\" | \"POST\", code Concrete<Int>}\n@main\nx = T {m \"GET\", code 200}\n",
        );
        assert_eq!(s["properties"]["m"], json!({"const":"GET"}));
        assert_eq!(s["properties"]["code"], json!({"const":200}));
    }

    #[test]
    fn optional_excluded_from_required() {
        let s = schema_for("meta T = {a! String, b String}\n@main\nx = T {a String, b? String}\n");
        let req = s["required"].as_array().unwrap();
        assert!(req.contains(&json!("a")));
        assert!(!req.contains(&json!("b")));
    }

    #[test]
    fn named_type_ref_populates_defs() {
        // An open field whose value is a bare meta name becomes a `$ref` into `$defs`.
        let s = schema_for("meta B = {x! String}\nmeta T = {b! B}\n@main\nm = T {b B}\n");
        assert_eq!(s["properties"]["b"], json!({"$ref":"#/$defs/B"}));
        assert_eq!(s["$defs"]["B"]["properties"]["x"], json!({"type":"string"}));
    }

    #[test]
    fn struct_carries_type_tag() {
        let s = schema_for("meta T = {a! String}\n@main\nm = T {a String}\n");
        assert_eq!(s["_type"], json!("T"));
        assert!(s.get("title").is_none());
    }

    #[test]
    fn identifier_variant_value_is_const_without_defs() {
        // `method POST` parses as BindingRef("POST"); POST is an identifier variant
        // (empty-closed-struct meta) → fixed tag, no `$defs` entry.
        let s = schema_for("meta E = {m! GET | POST | DELETE}\n@main\nx = E {m POST}\n");
        assert_eq!(s["properties"]["m"], json!({"const":"POST"}));
        assert!(s.get("$defs").is_none());
    }

    #[test]
    fn uniform_list_single_items() {
        let s = schema_for("meta T = {xs []String}\n@main\nx = T {xs [String, String]}\n");
        assert_eq!(s["properties"]["xs"]["items"], json!({"type":"string"}));
    }

    #[test]
    fn mixed_list_items_oneof() {
        // Two structurally different elements → single `items` with `oneOf`.
        let s = schema_for(
            "meta R = {status! Concrete<Int>, body {...}}\nmeta T = {rs []R}\n@main\nx = T {rs [{status 401}, {status 200, body {ok String}}]}\n",
        );
        let items = &s["properties"]["rs"]["items"];
        let one_of = items["oneOf"].as_array().expect("oneOf array");
        assert_eq!(one_of.len(), 2);
    }

    #[test]
    fn generated_field_marked() {
        let s = schema_for("meta T = {a! String, g! String}\n@main\nx = T {a String, g String*}\n");
        assert_eq!(s["properties"]["g"]["x-generated"], json!(true));
        assert!(s["properties"]["a"].get("x-generated").is_none());
    }

    #[test]
    fn type_decl_vs_instance_value_encoding() {
        // Open base-typed field (value is the type) → {type/format};
        // instance-fixed value → const. The split must survive in one schema.
        let s = schema_for(
            "meta T = {open! String, fixed! Concrete<String>}\n@main\nx = T {open String, fixed \"hi\"}\n",
        );
        assert_eq!(s["properties"]["open"], json!({"type":"string"}));
        assert_eq!(s["properties"]["fixed"], json!({"const":"hi"}));
    }
}
