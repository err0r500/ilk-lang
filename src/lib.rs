pub mod error;
pub mod ilk;
pub mod kli;
pub mod span;
pub mod validate;

use error::Diagnostic;
use ilk::TypeEnv;
use kli::KliFile;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct Compiler {
    ilk_cache: HashMap<PathBuf, (ilk::IlkFile, TypeEnv)>,
    kli_cache: HashMap<PathBuf, KliFile>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            ilk_cache: HashMap::new(),
            kli_cache: HashMap::new(),
        }
    }

    pub fn load_ilk(&mut self, path: &Path, src: &str) -> Result<&TypeEnv, Vec<Diagnostic>> {
        let file = ilk::parse_ilk(src, path)?;
        let env = ilk::resolve(&file, path)?;
        self.ilk_cache.insert(path.to_path_buf(), (file, env));
        Ok(&self.ilk_cache.get(path).unwrap().1)
    }

    pub fn load_kli(&mut self, path: &Path, src: &str) -> Result<&KliFile, Vec<Diagnostic>> {
        let file = kli::parse_kli(src, path)?;
        self.kli_cache.insert(path.to_path_buf(), file);
        Ok(self.kli_cache.get(path).unwrap())
    }

    pub fn validate(&self, ilk_path: &Path, kli_path: &Path) -> Result<(), Vec<Diagnostic>> {
        let (_, env) = self
            .ilk_cache
            .get(ilk_path)
            .ok_or_else(|| vec![Diagnostic::error(0..0, "ilk file not loaded", ilk_path)])?;

        let kli = self
            .kli_cache
            .get(kli_path)
            .ok_or_else(|| vec![Diagnostic::error(0..0, "kli file not loaded", kli_path)])?;

        let ctx = validate::ValidationContext::new(env, kli, kli_path);
        let mut errors = Vec::new();

        validate::validate_structural(&ctx, kli, &mut errors);
        validate::validate_source(&ctx, kli, &mut errors);
        validate::validate_constraints(&ctx, kli, &mut errors);

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    pub fn invalidate(&mut self, path: &Path) {
        self.ilk_cache.remove(path);
        self.kli_cache.remove(path);
    }
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

pub fn validate_files(ilk_path: &Path, kli_path: &Path) -> Result<(), Vec<Diagnostic>> {
    let ilk_src = std::fs::read_to_string(ilk_path).map_err(|e| {
        vec![Diagnostic::error(
            0..0,
            format!("Failed to read ilk file: {}", e),
            ilk_path,
        )]
    })?;

    let kli_src = std::fs::read_to_string(kli_path).map_err(|e| {
        vec![Diagnostic::error(
            0..0,
            format!("Failed to read kli file: {}", e),
            kli_path,
        )]
    })?;

    let mut compiler = Compiler::new();
    compiler.load_ilk(ilk_path, &ilk_src)?;
    compiler.load_kli(kli_path, &kli_src)?;
    compiler.validate(ilk_path, kli_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_dcb_board() {
        let result = validate_files(
            Path::new("examples/dcb-board-spec.ilk"),
            Path::new("examples/dcb-board-instance-valid.kli"),
        );
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }
}
