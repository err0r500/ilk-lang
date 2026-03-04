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
pub struct RawBlock {
    pub kind: Spanned<String>,
    pub name: Option<Spanned<String>>,
    pub body: Vec<Spanned<RawItem>>,
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
}
