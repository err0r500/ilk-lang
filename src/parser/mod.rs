pub mod common;
pub mod items;
pub mod types;
pub mod values;

use crate::ast::Comment;
use crate::error::Diagnostic;
use std::path::Path;

pub use types::type_expr;
pub use values::value;

fn extract_comments(src: &str) -> Vec<Comment> {
    let mut comments = Vec::new();
    let mut i = 0;
    let bytes = src.as_bytes();

    while i < bytes.len() {
        if i + 1 < bytes.len() && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            let start = i;
            i += 2;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            let text = src[start..i].to_string();
            comments.push(Comment {
                span: start..i,
                text,
            });
        } else {
            i += 1;
        }
    }

    comments
}

pub fn parse(src: &str, path: &Path) -> Result<crate::ast::File, Vec<Diagnostic>> {
    use chumsky::Parser as _;

    let comments = extract_comments(src);

    items::file()
        .parse(src)
        .into_result()
        .map(|mut f| {
            f.comments = comments;
            f
        })
        .map_err(|errs| {
            errs.into_iter()
                .map(|e| {
                    Diagnostic::error(e.span().into_range(), e.to_string(), path.to_path_buf())
                })
                .collect()
        })
}
