use crate::error::{Diagnostic, Severity};
use tower_lsp::lsp_types::{self, DiagnosticSeverity, Position, Range};

pub fn convert(diagnostics: &[Diagnostic], src: &str) -> Vec<lsp_types::Diagnostic> {
    diagnostics.iter().map(|d| to_lsp(d, src)).collect()
}

fn to_lsp(diag: &Diagnostic, src: &str) -> lsp_types::Diagnostic {
    let range = span_to_range(diag.span.start, diag.span.end, src);
    let severity = match diag.severity {
        Severity::Error => DiagnosticSeverity::ERROR,
        Severity::Warning => DiagnosticSeverity::WARNING,
    };

    lsp_types::Diagnostic {
        range,
        severity: Some(severity),
        message: diag.message.clone(),
        source: Some("ilk".to_string()),
        ..Default::default()
    }
}

pub fn span_to_range(start: usize, end: usize, src: &str) -> Range {
    Range {
        start: offset_to_position(start, src),
        end: offset_to_position(end, src),
    }
}

pub fn offset_to_position(offset: usize, src: &str) -> Position {
    let mut line = 0u32;
    let mut col = 0u32;

    for (i, ch) in src.char_indices() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    Position::new(line, col)
}

pub fn position_to_offset(pos: Position, src: &str) -> usize {
    let mut line = 0u32;
    let mut col = 0u32;

    for (i, ch) in src.char_indices() {
        if line == pos.line && col == pos.character {
            return i;
        }
        if ch == '\n' {
            if line == pos.line {
                return i;
            }
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }

    src.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_offset_to_position() {
        let src = "line1\nline2\nline3";
        assert_eq!(offset_to_position(0, src), Position::new(0, 0));
        assert_eq!(offset_to_position(5, src), Position::new(0, 5));
        assert_eq!(offset_to_position(6, src), Position::new(1, 0));
        assert_eq!(offset_to_position(10, src), Position::new(1, 4));
    }

    #[test]
    fn test_position_to_offset() {
        let src = "line1\nline2\nline3";
        assert_eq!(position_to_offset(Position::new(0, 0), src), 0);
        assert_eq!(position_to_offset(Position::new(1, 0), src), 6);
        assert_eq!(position_to_offset(Position::new(1, 4), src), 10);
    }
}
