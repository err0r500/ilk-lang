use crate::ast::Span;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub span: Span,
    pub message: String,
    pub expected: Vec<String>,
    pub found: Option<String>,
}

impl ParseError {
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
            expected: vec![],
            found: None,
        }
    }

    pub fn with_expected(mut self, expected: Vec<String>) -> Self {
        self.expected = expected;
        self
    }

    pub fn with_found(mut self, found: Option<String>) -> Self {
        self.found = found;
        self
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)?;
        if !self.expected.is_empty() {
            write!(f, " (expected: {})", self.expected.join(", "))?;
        }
        if let Some(found) = &self.found {
            write!(f, " (found: {})", found)?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {}

pub fn convert_errors(errs: Vec<chumsky::error::Rich<'_, char>>) -> Vec<ParseError> {
    errs.into_iter()
        .map(|e| {
            let span = e.span().start..e.span().end;
            let expected: Vec<String> = e
                .expected()
                .map(|exp| format!("{:?}", exp))
                .collect();
            let found = e.found().map(|c| c.to_string());
            ParseError::new(span, e.reason().to_string())
                .with_expected(expected)
                .with_found(found)
        })
        .collect()
}
