pub mod ast;
pub mod emit;
pub mod error;
pub mod formatter;
pub mod lsp;
pub mod parser;
pub mod resolve;
pub mod span;
pub mod validate;

use ast::File;
use error::Diagnostic;
use resolve::TypeEnv;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Compiler {
    cache: HashMap<PathBuf, (File, TypeEnv)>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &Path, src: &str) -> Result<&TypeEnv, Vec<Diagnostic>> {
        let file = parser::parse(src, path)?;
        let env = resolve::resolve(&file, path)?;
        self.cache.insert(path.to_path_buf(), (file, env));
        Ok(&self.cache.get(path).unwrap().1)
    }

    pub fn validate(&self, path: &Path) -> Result<(), Vec<Diagnostic>> {
        let (file, env) = self
            .cache
            .get(path)
            .ok_or_else(|| vec![Diagnostic::error(0..0, "file not loaded", path)])?;

        let ctx = validate::ValidationContext::new(env, path);
        let mut errors = Vec::new();

        validate::validate_structural(&ctx, file, &mut errors);
        validate::validate_source(&ctx, file, &mut errors);
        validate::validate_constraints(&ctx, file, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn invalidate(&mut self, path: &Path) {
        self.cache.remove(path);
    }

    pub fn get_file(&self, path: &Path) -> Option<&File> {
        self.cache.get(path).map(|(f, _)| f)
    }

    pub fn get_env(&self, path: &Path) -> Option<&TypeEnv> {
        self.cache.get(path).map(|(_, e)| e)
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse source code into an AST
pub fn parse(src: &str, path: &Path) -> Result<File, Vec<Diagnostic>> {
    parser::parse(src, path)
}

/// Compile source code (parse + resolve + validate)
pub fn compile(src: &str, path: &Path) -> Result<TypeEnv, Vec<Diagnostic>> {
    let file = parser::parse(src, path)?;
    let env = resolve::resolve(&file, path)?;
    let ctx = validate::ValidationContext::new(&env, path);
    let mut errors = Vec::new();
    validate::validate_structural(&ctx, &file, &mut errors);
    validate::validate_source(&ctx, &file, &mut errors);
    validate::validate_constraints(&ctx, &file, &mut errors);
    if errors.is_empty() {
        Ok(env)
    } else {
        Err(errors)
    }
}

/// Convenience function to validate a single file
pub fn validate_file(path: &Path) -> Result<(), Vec<Diagnostic>> {
    let src = std::fs::read_to_string(path).map_err(|e| {
        vec![Diagnostic::error(
            0..0,
            format!("Failed to read file: {}", e),
            path,
        )]
    })?;

    let mut compiler = Compiler::new();
    compiler.load(path, &src)?;
    compiler.validate(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dcb_board() {
        let result = validate_file(Path::new("examples/dcb-board.ilk"));
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }
}
