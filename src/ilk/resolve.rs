use crate::error::Diagnostic;
use crate::ilk::ast::*;
use crate::span::S;
use std::collections::{HashMap, HashSet};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct TypeEnv {
    pub blocks: HashMap<String, S<Block>>,
    pub main_block: Option<String>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            blocks: HashMap::new(),
            main_block: None,
        }
    }

    pub fn get(&self, name: &str) -> Option<&S<Block>> {
        self.blocks.get(name)
    }

    pub fn main(&self) -> Option<&S<Block>> {
        self.main_block.as_ref().and_then(|n| self.blocks.get(n))
    }
}

pub fn resolve(file: &IlkFile, path: &Path) -> Result<TypeEnv, Vec<Diagnostic>> {
    let mut env = TypeEnv::new();
    let mut errors = Vec::new();

    // Collect all blocks
    for block in &file.blocks {
        let name = &block.node.name.node;
        if env.blocks.contains_key(name) {
            errors.push(Diagnostic::error(
                block.node.name.span.clone(),
                format!("Duplicate block: {}", name),
                path,
            ));
        } else {
            env.blocks.insert(name.clone(), block.clone());
        }

        // Check for @main
        for ann in &block.node.annotations {
            if matches!(ann.node, Annotation::Main) {
                if env.main_block.is_some() {
                    errors.push(Diagnostic::error(
                        ann.span.clone(),
                        "Multiple @main annotations",
                        path,
                    ));
                } else {
                    env.main_block = Some(name.clone());
                }
            }
        }
    }

    // Check for unknown type references
    for block in &file.blocks {
        check_type_refs(&block.node.body, &env, path, &mut errors);
    }

    // Check for cycles
    check_cycles(&env, path, &mut errors);

    if errors.is_empty() {
        Ok(env)
    } else {
        Err(errors)
    }
}

fn check_type_refs(ty: &S<TypeExpr>, env: &TypeEnv, path: &Path, errors: &mut Vec<Diagnostic>) {
    match &ty.node {
        TypeExpr::Named(name) => {
            if !env.blocks.contains_key(name) {
                errors.push(Diagnostic::error(
                    ty.span.clone(),
                    format!("Unknown type: {}", name),
                    path,
                ));
            }
        }
        TypeExpr::Reference(name) => {
            if !env.blocks.contains_key(name) {
                errors.push(Diagnostic::error(
                    ty.span.clone(),
                    format!("Unknown type in reference: {}", name),
                    path,
                ));
            }
        }
        TypeExpr::Concrete(inner) => check_type_refs(inner, env, path, errors),
        TypeExpr::List(_, inner) => check_type_refs(inner, env, path, errors),
        TypeExpr::Union(variants) => {
            for v in variants {
                check_type_refs(v, env, path, errors);
            }
        }
        TypeExpr::Intersection(left, right) => {
            check_type_refs(left, env, path, errors);
            check_type_refs(right, env, path, errors);
        }
        TypeExpr::Struct(kind) => match kind {
            StructKind::Closed(fields) | StructKind::Open(fields) => {
                for field in fields {
                    check_type_refs(&field.node.ty, env, path, errors);
                }
            }
            StructKind::Anonymous(types) => {
                for ty in types.iter().flatten() {
                    check_type_refs(ty, env, path, errors);
                }
            }
        },
        _ => {}
    }
}

fn check_cycles(env: &TypeEnv, path: &Path, errors: &mut Vec<Diagnostic>) {
    let mut visited = HashSet::new();
    let mut in_stack = HashSet::new();

    for name in env.blocks.keys() {
        if !visited.contains(name) {
            check_cycles_dfs(name, env, path, &mut visited, &mut in_stack, errors);
        }
    }
}

fn check_cycles_dfs(
    name: &str,
    env: &TypeEnv,
    path: &Path,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    errors: &mut Vec<Diagnostic>,
) {
    visited.insert(name.to_string());
    in_stack.insert(name.to_string());

    if let Some(block) = env.blocks.get(name) {
        let deps = collect_direct_deps(&block.node.body);
        for dep in deps {
            if in_stack.contains(&dep) {
                errors.push(Diagnostic::error(
                    block.node.name.span.clone(),
                    format!("Cyclic reference: {} -> {}", name, dep),
                    path,
                ));
            } else if !visited.contains(&dep) {
                check_cycles_dfs(&dep, env, path, visited, in_stack, errors);
            }
        }
    }

    in_stack.remove(name);
}

fn collect_direct_deps(ty: &S<TypeExpr>) -> Vec<String> {
    let mut deps = Vec::new();
    collect_deps_inner(&ty.node, &mut deps);
    deps
}

fn collect_deps_inner(ty: &TypeExpr, deps: &mut Vec<String>) {
    match ty {
        TypeExpr::Named(name) => deps.push(name.clone()),
        TypeExpr::Reference(name) => deps.push(name.clone()),
        TypeExpr::Concrete(inner) => collect_deps_inner(&inner.node, deps),
        TypeExpr::List(_, inner) => collect_deps_inner(&inner.node, deps),
        TypeExpr::Union(variants) => {
            for v in variants {
                collect_deps_inner(&v.node, deps);
            }
        }
        TypeExpr::Intersection(left, right) => {
            collect_deps_inner(&left.node, deps);
            collect_deps_inner(&right.node, deps);
        }
        TypeExpr::Struct(kind) => match kind {
            StructKind::Closed(fields) | StructKind::Open(fields) => {
                for field in fields {
                    collect_deps_inner(&field.node.ty.node, deps);
                }
            }
            StructKind::Anonymous(types) => {
                for ty in types.iter().flatten() {
                    collect_deps_inner(&ty.node, deps);
                }
            }
        },
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ilk::parser::parse_ilk;

    fn resolve_str(s: &str) -> Result<TypeEnv, Vec<Diagnostic>> {
        let file = parse_ilk(s, Path::new("test.ilk")).unwrap();
        resolve(&file, Path::new("test.ilk"))
    }

    #[test]
    fn test_block_collection() {
        let env = resolve_str("Foo {x Int}").unwrap();
        assert!(env.blocks.contains_key("Foo"));
    }

    #[test]
    fn test_main_block() {
        let env = resolve_str("@main\nFoo {...}").unwrap();
        assert_eq!(env.main_block, Some("Foo".to_string()));
    }

    #[test]
    fn test_forward_refs() {
        let env = resolve_str("A B\nB {x Int}");
        assert!(env.is_ok());
    }

    #[test]
    fn test_cycles() {
        let result = resolve_str("A B\nB A");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Cyclic")));
    }

    #[test]
    fn test_multiple_main() {
        let result = resolve_str("@main\nA {}\n@main\nB {}");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Multiple @main")));
    }

    #[test]
    fn test_unknown_type() {
        let result = resolve_str("A Unknown");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Unknown type")));
    }
}
