use crate::span::S;

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Cardinality {
    Any,           // []
    Exact(usize),  // [N]
    AtLeast(usize), // [N..]
    AtMost(usize),  // [..M]
    Range(usize, usize), // [N..M]
}

#[derive(Debug, Clone, PartialEq)]
pub enum StructKind {
    Closed(Vec<S<Field>>),           // {x Int, y String}
    Open(Vec<S<Field>>),             // {...} or {...} & {x Int}
    Anonymous(Vec<Option<S<TypeExpr>>>), // {_}, {_ String}, {_, _}
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub name: S<String>,
    pub ty: S<TypeExpr>,
    pub annotations: Vec<S<Annotation>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpr {
    Base(BaseType),
    Concrete(Box<S<TypeExpr>>),
    LitString(String),
    LitInt(i64),
    LitBool(bool),
    Named(String),
    Struct(StructKind),
    List(Cardinality, Box<S<TypeExpr>>),
    Reference(String),
    Union(Vec<S<TypeExpr>>),
    Intersection(Box<S<TypeExpr>>, Box<S<TypeExpr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourcePath {
    Simple(String),
    Dotted(Vec<String>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Annotation {
    Main,
    Assoc(Vec<S<String>>),
    Source(Vec<S<SourcePath>>),
    Out,
    Constraint(S<ConstraintExpr>),
    Doc(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintExpr {
    Bool(bool),
    Var(String),
    FieldAccess(Box<S<ConstraintExpr>>, String),
    ForAll(String, String, Box<S<ConstraintExpr>>),
    ForAllExpr(Box<S<ConstraintExpr>>, String, Box<S<ConstraintExpr>>), // forall(expr, var => body)
    Exists(String, String, Box<S<ConstraintExpr>>),
    Unique(String, String, Box<S<ConstraintExpr>>),
    Count(String),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub name: S<String>,
    pub annotations: Vec<S<Annotation>>,
    pub body: S<TypeExpr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IlkFile {
    pub blocks: Vec<S<Block>>,
}
