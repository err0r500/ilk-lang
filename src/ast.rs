use crate::span::{Span, S};
use serde::Serialize;

// ============= Type-Level AST (formerly ilk/ast.rs) =============

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseType {
    Wildcard,
    Uuid,
    String,
    Int,
    Float,
    Bool,
    Date,
    Timestamp,
    Money,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Cardinality {
    Any,                 // []
    Exact(usize),        // [N]
    AtLeast(usize),      // [N..]
    AtMost(usize),       // [..M]
    Range(usize, usize), // [N..M]
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StructKind {
    Closed(Vec<S<Field>>),               // {x Int, y String}
    Open(Vec<S<Field>>),                 // {...} or {...} & {x Int}
    Anonymous(Vec<Option<S<TypeExpr>>>), // {_}, {_ String}, {_, _}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Field {
    pub name: S<String>,
    pub optional: bool,
    pub ty: S<TypeExpr>,
    pub annotations: Vec<S<Annotation>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypeExpr {
    Base(BaseType),
    Concrete(Box<S<TypeExpr>>),
    LitString(String),
    LitInt(i64),
    LitBool(bool),
    Named(String),
    RefinableRef(String), // -TypeName - allows refinement with concrete values
    Struct(StructKind),
    List(Cardinality, Box<S<TypeExpr>>),
    Reference(String),
    Union(Vec<S<TypeExpr>>),
    Intersection(Box<S<TypeExpr>>, Box<S<TypeExpr>>),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourcePath {
    Simple(String),
    Dotted(Vec<String>),
}

impl SourcePath {
    /// Returns the first segment of the path (the root name).
    pub fn root_name(&self) -> &str {
        match self {
            SourcePath::Simple(name) => name,
            SourcePath::Dotted(parts) => parts.first().expect("dotted path must not be empty"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Annotation {
    Main,
    Assoc(Vec<S<String>>),
    Source(Vec<S<SourcePath>>),
    Out,
    Constraint(S<ConstraintExpr>),
    Doc(String),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintExpr {
    Bool(bool),
    Var(String),
    FieldAccess(Box<S<ConstraintExpr>>, String),
    All(Box<S<ConstraintExpr>>, String, Box<S<ConstraintExpr>>),
    Exists(Box<S<ConstraintExpr>>, String, Box<S<ConstraintExpr>>),
    Unique(Box<S<ConstraintExpr>>, String, Box<S<ConstraintExpr>>),
    Count(Box<S<ConstraintExpr>>),
    Assoc(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    TemplateVars(Box<S<ConstraintExpr>>),
    Keys(Box<S<ConstraintExpr>>),
    And(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Or(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Not(Box<S<ConstraintExpr>>),
    Eq(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Ne(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    In(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Lt(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Le(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Gt(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Ge(Box<S<ConstraintExpr>>, Box<S<ConstraintExpr>>),
    Int(i64),
    IsType(Box<S<ConstraintExpr>>, String), // isType(expr, TypeName)
}

// ============= Instance-Level AST (formerly kli/ast.rs) =============

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FieldOrigin {
    None,                       // No annotation - implicit name match
    Generated,                  // Type*
    Mapped(Vec<String>),        // Type = path.to.field
    Computed(Vec<Vec<String>>), // Type = compute(path1, path2)
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct InstanceField {
    pub name: S<String>,
    pub optional: bool,
    pub assocs: Vec<S<String>>, // inline assocs: fieldName <inst1, inst2> {...}
    pub value: S<Value>,
    pub origin: FieldOrigin,
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Value {
    TypeRef(String),                                           // String, Int, etc.
    LitString(String),                                         // "hello"
    LitInt(i64),                                               // 42
    LitBool(bool),                                             // true/false
    BindingRef(String),                                        // someBinding
    Struct(Vec<S<InstanceField>>),                             // {x Int, y String}
    List(Vec<S<ListElement>>),                                 // [a, b, c]
    Variant(String, Box<S<Value>>),                            // VariantName body
    Refinement(String, Vec<S<String>>, Vec<S<InstanceField>>), // binding <assocs> & {fields}
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ListElement {
    Value(Value),
    BindingRef(String),
    Refinement(String, Vec<S<String>>, Vec<S<InstanceField>>), // binding <assocs> & {field origins}
}

// ============= Unified Top-Level Items =============

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TypeDecl {
    pub name: S<String>,
    pub annotations: Vec<S<Annotation>>,
    pub body: S<TypeExpr>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Instance {
    pub name: S<String>,
    pub type_name: S<String>,
    pub assocs: Vec<S<String>>,
    pub body: S<Value>,
    pub annotations: Vec<S<Annotation>>, // can have @main
    pub doc: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Import {
    pub path: S<String>,
    pub alias: Option<S<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    TypeDecl(TypeDecl),
    Instance(Instance),
    Import(Import),
}

// NB : we keep comments in the AST for code formatting

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Comment {
    pub span: Span,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct File {
    pub items: Vec<S<Item>>,
    pub comments: Vec<Comment>,
}

impl File {
    pub fn type_decls(&self) -> impl Iterator<Item = &TypeDecl> {
        self.items.iter().filter_map(|item| match &item.node {
            Item::TypeDecl(t) => Some(t),
            _ => None,
        })
    }

    pub fn instances(&self) -> impl Iterator<Item = &Instance> {
        self.items.iter().filter_map(|item| match &item.node {
            Item::Instance(i) => Some(i),
            _ => None,
        })
    }

    pub fn imports(&self) -> impl Iterator<Item = &Import> {
        self.items.iter().filter_map(|item| match &item.node {
            Item::Import(i) => Some(i),
            _ => None,
        })
    }
}
