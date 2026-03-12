use crate::span::S;

#[derive(Debug, Clone, PartialEq)]
pub enum FieldOrigin {
    None,                     // No annotation - implicit name match
    Generated,                // Type*
    Mapped(Vec<String>),      // Type = path.to.field
    Computed(Vec<Vec<String>>), // Type = compute(path1, path2)
}

#[derive(Debug, Clone, PartialEq)]
pub struct KliField {
    pub name: S<String>,
    pub optional: bool,
    pub value: S<KliValue>,
    pub origin: FieldOrigin,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KliValue {
    TypeRef(String),                    // String, Int, etc.
    LitString(String),                  // "hello"
    LitInt(i64),                        // 42
    LitBool(bool),                      // true/false
    BindingRef(String),                 // someBinding
    Struct(Vec<S<KliField>>),          // {x Int, y String}
    List(Vec<S<KliListElement>>),      // [a, b, c]
    Variant(String, Box<S<KliValue>>), // VariantName body
}

#[derive(Debug, Clone, PartialEq)]
pub enum KliListElement {
    Value(KliValue),
    BindingRef(String),
    Refinement(String, Vec<S<KliField>>), // binding & {field origins}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Binding {
    pub name: S<String>,
    pub type_name: S<String>,
    pub assocs: Vec<S<String>>,
    pub body: S<KliValue>,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KliFile {
    pub bindings: Vec<S<Binding>>,
}
