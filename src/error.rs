use crate::span::Span;
use ariadne::{Color, Label, Report, ReportKind, Source};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
}

/// Stable machine-readable kind of a diagnostic. Tests assert on this instead
/// of matching message substrings, so rewording a message is not a breaking
/// change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosticCode {
    // parsing / loading
    Parse,
    ImportNotFound,
    CircularImport,
    ConflictingImport,
    FileNotLoaded,
    FileRead,
    // resolve
    DuplicateMeta,
    DuplicateInstance,
    MultipleMain,
    UnknownType,
    UnknownMetaInReference,
    CyclicReference,
    // structural
    TypeMismatch,
    ConcreteRequiresLiteral,
    LiteralMismatch,
    LiteralForOpenType,
    DuplicateField,
    ExtraField,
    MissingRequiredField,
    ReferenceTypeMismatch,
    RefinementTypeMismatch,
    NotRefinable,
    InvalidRefinement,
    UnknownInstance,
    UnknownField,
    UnknownVariant,
    UnionMismatch,
    WrongFieldCount,
    ListCardinalityMismatch,
    // @source
    SourcePathNotAllowed,
    SourcePathNotFound,
    NoSourceFound,
    AmbiguousSource,
    OptionalSourceForMandatoryField,
    SourceTypeMismatch,
    // constraints
    ConstraintFailed,
    ConstraintNotBoolean,
    ConstraintEvalError,
    /// Catch-all for diagnostics without a dedicated code yet.
    Other,
}

#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: DiagnosticCode,
    pub span: Span,
    pub message: String,
    pub file: PathBuf,
}

impl Serialize for Diagnostic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut s = serializer.serialize_struct("Diagnostic", 6)?;
        s.serialize_field("severity", &self.severity)?;
        s.serialize_field("code", &self.code)?;
        s.serialize_field("start", &self.span.start)?;
        s.serialize_field("end", &self.span.end)?;
        s.serialize_field("message", &self.message)?;
        s.serialize_field("file", &self.file)?;
        s.end()
    }
}

impl Diagnostic {
    pub fn error(span: Span, message: impl Into<String>, file: impl Into<PathBuf>) -> Self {
        Self {
            severity: Severity::Error,
            code: DiagnosticCode::Other,
            span,
            message: message.into(),
            file: file.into(),
        }
    }

    pub fn warning(span: Span, message: impl Into<String>, file: impl Into<PathBuf>) -> Self {
        Self {
            severity: Severity::Warning,
            code: DiagnosticCode::Other,
            span,
            message: message.into(),
            file: file.into(),
        }
    }

    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = code;
        self
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
