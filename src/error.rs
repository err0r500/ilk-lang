use crate::span::Span;
use ariadne::{Color, Label, Report, ReportKind, Source};
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Error,
    Warning,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub span: Span,
    pub message: String,
    pub file: PathBuf,
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>, file: impl Into<PathBuf>) -> Self {
        Self {
            severity: Severity::Error,
            span,
            message: message.into(),
            file: file.into(),
        }
    }

    pub fn warning(span: Span, message: impl Into<String>, file: impl Into<PathBuf>) -> Self {
        Self {
            severity: Severity::Warning,
            span,
            message: message.into(),
            file: file.into(),
        }
    }

    pub fn to_report(&self, src: &str) -> String {
        let kind = match self.severity {
            Severity::Error => ReportKind::Error,
            Severity::Warning => ReportKind::Warning,
        };
        let color = match self.severity {
            Severity::Error => Color::Red,
            Severity::Warning => Color::Yellow,
        };
        let filename = self.file.display().to_string();

        let mut buf = Vec::new();
        Report::build(kind, &filename, self.span.start)
            .with_message(&self.message)
            .with_label(
                Label::new((&filename, self.span.clone()))
                    .with_message(&self.message)
                    .with_color(color),
            )
            .finish()
            .write((&filename, Source::from(src)), &mut buf)
            .unwrap();
        String::from_utf8(buf).unwrap()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Parse(Vec<Diagnostic>),
    Resolve(Vec<Diagnostic>),
    Validate(Vec<Diagnostic>),
}

impl Error {
    pub fn diagnostics(&self) -> &[Diagnostic] {
        match self {
            Error::Parse(d) | Error::Resolve(d) | Error::Validate(d) => d,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_renders_with_ariadne() {
        let diag = Diagnostic::error(0..5, "test error", "test.ilk");
        let report = diag.to_report("hello world");
        assert!(report.contains("test error"));
    }
}
