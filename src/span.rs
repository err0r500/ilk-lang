use chumsky::span::SimpleSpan;
use std::ops::Range;

pub type Span = Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }

    pub fn from_simple(node: T, span: SimpleSpan) -> Self {
        Self {
            node,
            span: span.into_range(),
        }
    }

    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Spanned<U> {
        Spanned {
            node: f(self.node),
            span: self.span,
        }
    }

    pub fn as_ref(&self) -> Spanned<&T> {
        Spanned {
            node: &self.node,
            span: self.span.clone(),
        }
    }
}

pub type S<T> = Spanned<T>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spanned_preserves_range() {
        let s = Spanned::new("x", 0..1);
        assert_eq!(s.span, 0..1);
    }

    #[test]
    fn spanned_map() {
        let s = Spanned::new(1, 0..1);
        let s2 = s.map(|x| x + 1);
        assert_eq!(s2.node, 2);
        assert_eq!(s2.span, 0..1);
    }
}
