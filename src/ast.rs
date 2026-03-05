use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T>(pub T, pub Span);

impl<T> Spanned<T> {
    pub fn new(value: T, span: Span) -> Self {
        Self(value, span)
    }

    pub fn value(&self) -> &T {
        &self.0
    }

    pub fn span(&self) -> &Span {
        &self.1
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned(f(self.0), self.1)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TagRef {
    Ident(String),
    String(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBody {
    Items(Vec<Spanned<RawItem>>),
    Value(Spanned<RawValue>),
}

impl BlockBody {
    pub fn items(&self) -> &[Spanned<RawItem>] {
        match self {
            BlockBody::Items(items) => items,
            BlockBody::Value(_) => &[],
        }
    }

    pub fn value(&self) -> Option<&Spanned<RawValue>> {
        match self {
            BlockBody::Items(_) => None,
            BlockBody::Value(v) => Some(v),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawBlock {
    pub kind: Spanned<String>,
    pub name: Option<Spanned<String>>,
    pub associations: Vec<Spanned<TagRef>>,
    pub body: BlockBody,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawItem {
    Field(RawField),
    Block(RawBlock),
}

#[derive(Debug, Clone, PartialEq)]
pub struct RawField {
    pub name: Spanned<String>,
    pub optional: bool,
    pub generated: bool,
    pub value: Spanned<RawValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeRefinement {
    pub base: String,
    pub overrides: Vec<(Spanned<String>, Spanned<RawValue>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Cardinality {
    ZeroOrMore,
    Exactly(u32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RawValue {
    Type(String),
    Ident(String),
    String(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    List(Vec<Spanned<RawValue>>),
    Object(Vec<(Spanned<String>, Spanned<RawValue>)>),
    Ref(Vec<String>),
    Wildcard,
    TypeRefinement(TypeRefinement),
    WildcardObject {
        cardinality: Cardinality,
        value: Box<Spanned<RawValue>>,
    },
    Union(Vec<Spanned<RawValue>>),
    Intersection(Vec<Spanned<RawValue>>),
    /// Concrete<T> - expects a concrete value of type T in schema
    Concrete(Box<Spanned<RawValue>>),
}

/// Constraint expression for @constraint annotations
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintExpr {
    ForAll {
        field: String,
        var: String,
        body: Box<ConstraintExpr>,
    },
    Exists {
        field: String,
        var: String,
        body: Box<ConstraintExpr>,
    },
    Assoc {
        subject: String,
        target: String,
    },
    And(Box<ConstraintExpr>, Box<ConstraintExpr>),
    Or(Box<ConstraintExpr>, Box<ConstraintExpr>),
    Not(Box<ConstraintExpr>),
}
