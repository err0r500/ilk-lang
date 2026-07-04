pub mod ast;
pub mod emit_jsonschema;
pub mod emit_schema;
pub mod error;
pub mod formatter;
#[cfg(feature = "cli")]
pub mod lsp;
pub mod parser;
pub mod resolve;
pub mod span;
pub mod validate;

#[cfg(feature = "wasm")]
pub mod wasm;

use ast::File;
use error::{Diagnostic, DiagnosticCode};
use resolve::TypeEnv;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

pub struct Compiler {
    cache: HashMap<PathBuf, (File, TypeEnv, Vec<Diagnostic>)>,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn load(&mut self, path: &Path, src: &str) -> Result<&TypeEnv, Vec<Diagnostic>> {
        let file = parser::parse(src, path)?;
        let (imported_env, import_errors) =
            self.load_imports_from_file(&file, path, &mut HashSet::new())?;
        let (env, mut resolve_errors) = resolve::resolve_with_imports(&file, path, imported_env);
        resolve_errors.extend(import_errors);
        self.cache
            .insert(path.to_path_buf(), (file, env, resolve_errors));
        Ok(&self.cache.get(path).unwrap().1)
    }

    /// Load a file from disk, recursively loading its imports.
    pub fn load_file(&mut self, path: &Path) -> Result<&TypeEnv, Vec<Diagnostic>> {
        let canonical = path.canonicalize().map_err(|e| {
            vec![
                Diagnostic::error(0..0, format!("Cannot resolve path: {}", e), path)
                    .with_code(DiagnosticCode::FileRead),
            ]
        })?;
        if self.cache.contains_key(&canonical) {
            return Ok(&self.cache.get(&canonical).unwrap().1);
        }
        self.load_file_recursive(&canonical, &mut HashSet::new())
    }

    fn load_file_recursive(
        &mut self,
        path: &Path,
        loading: &mut HashSet<PathBuf>,
    ) -> Result<&TypeEnv, Vec<Diagnostic>> {
        if self.cache.contains_key(path) {
            return Ok(&self.cache.get(path).unwrap().1);
        }
        if !loading.insert(path.to_path_buf()) {
            return Err(vec![Diagnostic::error(
                0..0,
                format!("Circular import: {}", path.display()),
                path,
            )
            .with_code(DiagnosticCode::CircularImport)]);
        }

        let src = std::fs::read_to_string(path).map_err(|e| {
            vec![
                Diagnostic::error(0..0, format!("Failed to read file: {}", e), path)
                    .with_code(DiagnosticCode::FileRead),
            ]
        })?;
        let file = parser::parse(&src, path)?;
        let (imported_env, import_errors) = self.load_imports_from_file(&file, path, loading)?;
        let (env, mut resolve_errors) = resolve::resolve_with_imports(&file, path, imported_env);
        resolve_errors.extend(import_errors);
        self.cache
            .insert(path.to_path_buf(), (file, env, resolve_errors));
        loading.remove(path);
        Ok(&self.cache.get(path).unwrap().1)
    }

    fn load_imports_from_file(
        &mut self,
        file: &File,
        file_path: &Path,
        loading: &mut HashSet<PathBuf>,
    ) -> Result<(TypeEnv, Vec<Diagnostic>), Vec<Diagnostic>> {
        let mut merged = TypeEnv::new();
        let mut conflicts = Vec::new();
        let dir = file_path.parent().unwrap_or(Path::new("."));

        for import in file.imports() {
            let import_path = dir.join(&import.path.node).canonicalize().map_err(|e| {
                vec![Diagnostic::error(
                    import.path.span.clone(),
                    format!("Cannot resolve import '{}': {}", import.path.node, e),
                    file_path,
                )
                .with_code(DiagnosticCode::ImportNotFound)]
            })?;
            self.load_file_recursive(&import_path, loading)?;
            let (_, imported_env, _) = self.cache.get(&import_path).unwrap();
            for (name, ty) in &imported_env.metas {
                match merged.metas.entry(name.clone()) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(ty.clone());
                    }
                    // The same declaration reachable through two imports (diamond)
                    // is fine; two different declarations under one name is not.
                    std::collections::hash_map::Entry::Occupied(e) => {
                        if *e.get() != *ty {
                            conflicts.push(Diagnostic::error(
                                import.path.span.clone(),
                                format!("Conflicting import: meta '{}' is defined differently in another import", name),
                                file_path,
                            ).with_code(DiagnosticCode::ConflictingImport));
                        }
                    }
                }
            }
            for (name, inst) in &imported_env.instances {
                match merged.instances.entry(name.clone()) {
                    std::collections::hash_map::Entry::Vacant(e) => {
                        e.insert(inst.clone());
                    }
                    std::collections::hash_map::Entry::Occupied(e) => {
                        if *e.get() != *inst {
                            conflicts.push(Diagnostic::error(
                                import.path.span.clone(),
                                format!("Conflicting import: instance '{}' is defined differently in another import", name),
                                file_path,
                            ).with_code(DiagnosticCode::ConflictingImport));
                        }
                    }
                }
                merged
                    .instance_files
                    .entry(name.clone())
                    .or_insert_with(|| {
                        imported_env
                            .instance_files
                            .get(name)
                            .cloned()
                            .unwrap_or_else(|| import_path.clone())
                    });
            }
        }

        Ok((merged, conflicts))
    }

    pub fn validate(&self, path: &Path) -> Result<(), Vec<Diagnostic>> {
        if !self.cache.contains_key(path) {
            return Err(vec![Diagnostic::error(0..0, "file not loaded", path)
                .with_code(DiagnosticCode::FileNotLoaded)]);
        }

        let mut errors = Vec::new();
        for (file_path, (file, env, resolve_errors)) in &self.cache {
            errors.extend(resolve_errors.iter().cloned());
            let ctx = validate::ValidationContext::new(env, file_path);
            for inst in file.instances() {
                errors.extend(validate::validate_structural(&ctx, inst));
                errors.extend(validate::validate_source(&ctx, inst));
                errors.extend(validate::validate_constraints(&ctx, inst));
            }
        }

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
        self.cache.get(path).map(|(f, _, _)| f)
    }

    pub fn get_env(&self, path: &Path) -> Option<&TypeEnv> {
        self.cache.get(path).map(|(_, e, _)| e)
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
    let (env, mut errors) = resolve::resolve(&file, path);
    let ctx = validate::ValidationContext::new(&env, path);
    for inst in file.instances() {
        errors.extend(validate::validate_structural(&ctx, inst));
        errors.extend(validate::validate_source(&ctx, inst));
        errors.extend(validate::validate_constraints(&ctx, inst));
    }
    if errors.is_empty() {
        Ok(env)
    } else {
        Err(errors)
    }
}

/// Convenience function to validate a single file (with imports)
pub fn validate_file(path: &Path) -> Result<(), Vec<Diagnostic>> {
    let canonical = path.canonicalize().map_err(|e| {
        vec![
            Diagnostic::error(0..0, format!("Cannot resolve path: {}", e), path)
                .with_code(DiagnosticCode::FileRead),
        ]
    })?;
    let mut compiler = Compiler::new();
    compiler.load_file(&canonical)?;
    compiler.validate(&canonical)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect_ilk_files(dir: &Path, out: &mut Vec<PathBuf>) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                collect_ilk_files(&path, out);
            } else if path.extension().is_some_and(|e| e == "ilk") {
                out.push(path);
            }
        }
    }

    /// Work-in-progress examples that don't validate yet (also commented out of
    /// cart/main.ilk). They must still parse; remove entries as they are fixed.
    const KNOWN_INVALID_EXAMPLES: &[&str] = &[
        "examples/cart/viewChangedPrices.ilk",
        "examples/cart/viewOpenCartsWithProducts.ilk",
        "examples/cart/viewProductsInventories.ilk",
    ];

    #[test]
    fn test_validate_all_examples() {
        let mut files = Vec::new();
        collect_ilk_files(Path::new("examples"), &mut files);
        files.sort();
        assert!(!files.is_empty(), "no examples found");
        let mut failures = Vec::new();
        for file in &files {
            let known_invalid = KNOWN_INVALID_EXAMPLES
                .iter()
                .any(|k| file.ends_with(Path::new(k).file_name().unwrap()));
            let result = validate_file(file);
            match result {
                Ok(()) if known_invalid => failures.push(format!(
                    "{}: now validates — remove it from KNOWN_INVALID_EXAMPLES",
                    file.display()
                )),
                Err(errs) if !known_invalid => {
                    failures.push(format!("{}: {:?}", file.display(), errs))
                }
                _ => {}
            }
            // Known-invalid files must at least keep parsing.
            let src = std::fs::read_to_string(file).unwrap();
            if let Err(errs) = parse(&src, file) {
                failures.push(format!("{}: parse error {:?}", file.display(), errs));
            }
        }
        assert!(failures.is_empty(), "{}", failures.join("\n"));
    }

    #[test]
    fn test_validate_dcb_board() {
        let result = validate_file(Path::new("examples/dcb-board.ilk"));
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }

    #[test]
    fn test_validate_cart_with_imports() {
        let result = validate_file(Path::new("examples/cart/main.ilk"));
        assert!(result.is_ok(), "Validation failed: {:?}", result.err());
    }

    #[test]
    fn snapshot_diagnostic_messages() {
        // One bad snippet per diagnostic family. The codes are asserted in the
        // per-validator tests; this snapshot pins the rendered wording so a
        // reword is an intentional review, not a broken substring assert.
        let cases: &[&str] = &[
            "meta A = {x Int}\nmeta A = {y Int}",
            "meta A = {}\nmeta B = {}\n@main\na = A {}\n@main\nb = B {}",
            "meta A = Unknown",
            "meta A = &Unknown",
            "meta A = B\nmeta B = A",
            "meta Foo = {x Int}\nfoo = Foo {x String}",
            "meta Foo = {x Concrete<Int>}\nfoo = Foo {x Int}",
            "meta Foo = {x \"a\"}\nfoo = Foo {x \"b\"}",
            "meta Foo = {x String}\nfoo = Foo {x \"lit\"}",
            "meta Foo = {x Int}\nfoo = Foo {x Int, y Int}",
            "meta Foo = {x! Int}\nfoo = Foo {}",
            "meta Foo = {x Int}\nfoo = Foo {x Int, x Int}",
            "meta Status = Pending | Active\nmeta P = {s! Status}\np = P {s \"other\"}",
            "meta I = {x Int}\nmeta F = {items [3]I}\ni = I {x Int}\nf = F {items [i]}",
            "meta E = {...}\nmeta C = {emits []E}\ne = E {id String}\nc = C {emits [e & {id \"x\"}]}",
            "meta E = {id String}\nmeta C = {fields {...}\n@source [fields]\nemits E}\nc = C {fields {a Int}\nemits {id String = other.id}}",
            "meta E = {id String}\nmeta C = {fields {...}\n@source [fields]\nemits E}\nother = E {id String}\nc = C {fields {x Int}\nemits other}",
            "meta Foo = {\n@constraint x > 10\nx Int\n}\nfoo = Foo {x 5}",
            "meta Foo = {\n@constraint y > 0\nx Int\n}\nfoo = Foo {x 1}",
        ];
        let mut rendered = Vec::new();
        for (i, src) in cases.iter().enumerate() {
            let path = PathBuf::from(format!("case{}.ilk", i));
            let errs = match compile(src, &path) {
                Ok(_) => vec!["<no diagnostics>".to_string()],
                Err(errs) => errs
                    .iter()
                    .map(|e| format!("{:?}: {}", e.code, e.message))
                    .collect(),
            };
            rendered.push(format!("--- {:?}\n{}", src, errs.join("\n")));
        }
        insta::assert_snapshot!(rendered.join("\n\n"));
    }

    #[test]
    fn test_conflicting_imports_diagnosed() {
        let dir = std::env::temp_dir().join(format!("ilk-conflict-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("a.ilk"), "meta Shared = {x Int}\n").unwrap();
        std::fs::write(dir.join("b.ilk"), "meta Shared = {y String}\n").unwrap();
        std::fs::write(
            dir.join("main.ilk"),
            "import \"./a.ilk\"\nimport \"./b.ilk\"\n",
        )
        .unwrap();

        let result = validate_file(&dir.join("main.ilk"));
        std::fs::remove_dir_all(&dir).ok();
        let errors = result.expect_err("conflicting imports must be diagnosed");
        assert!(
            errors
                .iter()
                .any(|e| e.message.contains("Conflicting import: meta 'Shared'")),
            "{:?}",
            errors
        );
    }

    #[test]
    fn test_diamond_imports_ok() {
        let dir = std::env::temp_dir().join(format!("ilk-diamond-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("base.ilk"), "meta Shared = {x Int}\n").unwrap();
        std::fs::write(
            dir.join("a.ilk"),
            "import \"./base.ilk\"\nmeta A = {s Shared}\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("b.ilk"),
            "import \"./base.ilk\"\nmeta B = {s Shared}\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("main.ilk"),
            "import \"./a.ilk\"\nimport \"./b.ilk\"\n",
        )
        .unwrap();

        let result = validate_file(&dir.join("main.ilk"));
        std::fs::remove_dir_all(&dir).ok();
        assert!(
            result.is_ok(),
            "diamond imports must not conflict: {:?}",
            result.err()
        );
    }
}
