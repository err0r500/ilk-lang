use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::{Span, S};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct TypeEnv {
    pub types: HashMap<String, S<TypeDecl>>,
    pub instances: HashMap<String, S<Instance>>,
    pub instance_files: HashMap<String, PathBuf>,
    pub main_instance: Option<String>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            instances: HashMap::new(),
            instance_files: HashMap::new(),
            main_instance: None,
        }
    }

    pub fn get_type(&self, name: &str) -> Option<&S<TypeDecl>> {
        self.types.get(name)
    }

    pub fn get_instance(&self, name: &str) -> Option<&S<Instance>> {
        self.instances.get(name)
    }

    pub fn get_instance_file(&self, name: &str) -> Option<&Path> {
        self.instance_files.get(name).map(|p| p.as_path())
    }

    pub fn main(&self) -> Option<&S<Instance>> {
        self.main_instance
            .as_ref()
            .and_then(|n| self.instances.get(n))
    }
}

impl Default for TypeEnv {
    fn default() -> Self {
        Self::new()
    }
}

pub fn resolve(file: &File, path: &Path) -> Result<TypeEnv, Vec<Diagnostic>> {
    resolve_with_imports(file, path, TypeEnv::new())
}

pub fn resolve_with_imports(
    file: &File,
    path: &Path,
    imported_env: TypeEnv,
) -> Result<TypeEnv, Vec<Diagnostic>> {
    let mut env = imported_env;
    let mut errors = Vec::new();

    // Collect all type declarations
    for item in &file.items {
        if let Item::TypeDecl(decl) = &item.node {
            let name = &decl.name.node;
            if env.types.contains_key(name) {
                errors.push(Diagnostic::error(
                    decl.name.span.clone(),
                    format!("Duplicate type: {}", name),
                    path,
                ));
            } else {
                env.types
                    .insert(name.clone(), S::new(decl.clone(), item.span.clone()));
            }
        }
    }

    // Auto-register implicit marker types from union variants
    let mut implicit_types: Vec<(String, Span)> = Vec::new();
    for item in &file.items {
        if let Item::TypeDecl(decl) = &item.node {
            collect_implicit_union_variants(&decl.body, &env, &mut implicit_types);
        }
    }
    for (name, span) in implicit_types {
        if !env.types.contains_key(&name) && !is_base_type(&name) {
            let decl = TypeDecl {
                name: S::new(name.clone(), span.clone()),
                annotations: Vec::new(),
                body: S::new(
                    TypeExpr::Struct(StructKind::Closed(Vec::new())),
                    span.clone(),
                ),
            };
            env.types.insert(name, S::new(decl, span));
        }
    }

    // Collect all instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            let name = &inst.name.node;
            if env.instances.contains_key(name) {
                errors.push(Diagnostic::error(
                    inst.name.span.clone(),
                    format!("Duplicate instance: {}", name),
                    path,
                ));
            } else {
                env.instances
                    .insert(name.clone(), S::new(inst.clone(), item.span.clone()));
                env.instance_files.insert(name.clone(), path.to_path_buf());
            }

            // Check for @main
            for ann in &inst.annotations {
                if matches!(ann.node, Annotation::Main) {
                    if env.main_instance.is_some() {
                        errors.push(Diagnostic::error(
                            ann.span.clone(),
                            "Multiple @main annotations",
                            path,
                        ));
                    } else {
                        env.main_instance = Some(name.clone());
                    }
                }
            }
        }
    }

    // Check for unknown type references in type declarations
    for item in &file.items {
        if let Item::TypeDecl(decl) = &item.node {
            check_type_refs(&decl.body, &env, path, &mut errors);
        }
    }

    // Check for unknown type references in instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            let type_name = &inst.type_name.node;
            if !env.types.contains_key(type_name) && !is_base_type(type_name) {
                errors.push(Diagnostic::error(
                    inst.type_name.span.clone(),
                    format!("Unknown type: {}", type_name),
                    path,
                ));
            }
        }
    }

    // Check for cycles in type definitions
    check_cycles(&env, path, &mut errors);

    if errors.is_empty() {
        Ok(env)
    } else {
        Err(errors)
    }
}

fn is_base_type(name: &str) -> bool {
    matches!(
        name,
        "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
    )
}

fn collect_implicit_union_variants(ty: &S<TypeExpr>, env: &TypeEnv, out: &mut Vec<(String, Span)>) {
    match &ty.node {
        TypeExpr::Union(variants) => {
            for v in variants {
                if let TypeExpr::Named(name) = &v.node {
                    if !env.types.contains_key(name) && !is_base_type(name) {
                        out.push((name.clone(), v.span.clone()));
                    }
                }
                collect_implicit_union_variants(v, env, out);
            }
        }
        TypeExpr::Concrete(inner) => collect_implicit_union_variants(inner, env, out),
        TypeExpr::List(_, inner) => collect_implicit_union_variants(inner, env, out),
        TypeExpr::Intersection(left, right) => {
            collect_implicit_union_variants(left, env, out);
            collect_implicit_union_variants(right, env, out);
        }
        TypeExpr::Struct(kind) => match kind {
            StructKind::Closed(fields) | StructKind::Open(fields) => {
                for field in fields {
                    collect_implicit_union_variants(&field.node.ty, env, out);
                }
            }
            StructKind::Anonymous(types) => {
                for ty in types.iter().flatten() {
                    collect_implicit_union_variants(ty, env, out);
                }
            }
        },
        _ => {}
    }
}

fn check_type_refs(ty: &S<TypeExpr>, env: &TypeEnv, path: &Path, errors: &mut Vec<Diagnostic>) {
    match &ty.node {
        TypeExpr::Named(name) => {
            if !env.types.contains_key(name) {
                errors.push(Diagnostic::error(
                    ty.span.clone(),
                    format!("Unknown type: {}", name),
                    path,
                ));
            }
        }
        TypeExpr::Reference(name) => {
            if !env.types.contains_key(name) {
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

    for name in env.types.keys() {
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

    if let Some(decl) = env.types.get(name) {
        let deps = collect_direct_deps(&decl.node.body);
        for dep in deps {
            if in_stack.contains(&dep) {
                errors.push(Diagnostic::error(
                    decl.node.name.span.clone(),
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
    use crate::parser::parse;

    fn resolve_str(s: &str) -> Result<TypeEnv, Vec<Diagnostic>> {
        let file = parse(s, Path::new("test.ilk")).unwrap();
        resolve(&file, Path::new("test.ilk"))
    }

    #[test]
    fn test_type_collection() {
        let env = resolve_str("type Foo = {x Int}").unwrap();
        assert!(env.types.contains_key("Foo"));
    }

    #[test]
    fn test_instance_collection() {
        let env = resolve_str("type Foo = {x Int}\nfoo = Foo {x Int}").unwrap();
        assert!(env.instances.contains_key("foo"));
    }

    #[test]
    fn test_main_instance() {
        let env = resolve_str("type Foo = {...}\n@main\nfoo = Foo {x Int}").unwrap();
        assert_eq!(env.main_instance, Some("foo".to_string()));
    }

    #[test]
    fn test_forward_refs() {
        let env = resolve_str("type A = B\ntype B = {x Int}");
        assert!(env.is_ok());
    }

    #[test]
    fn test_cycles() {
        let result = resolve_str("type A = B\ntype B = A");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Cyclic")));
    }

    #[test]
    fn test_multiple_main() {
        let result = resolve_str("type A = {}\ntype B = {}\n@main\na = A {}\n@main\nb = B {}");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Multiple @main")));
    }

    #[test]
    fn test_unknown_type() {
        let result = resolve_str("type A = Unknown");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Unknown type")));
    }

    #[test]
    fn test_unknown_instance_type() {
        let result = resolve_str("foo = Unknown {x Int}");
        assert!(result.is_err());
        let errs = result.unwrap_err();
        assert!(errs.iter().any(|e| e.message.contains("Unknown type")));
    }

    #[test]
    fn test_implicit_union_marker_types() {
        let env = resolve_str(
            "type Status = Pending | Active | Archived\ntype Process = { status! Status }",
        )
        .unwrap();
        assert!(env.types.contains_key("Status"));
        assert!(env.types.contains_key("Pending"));
        assert!(env.types.contains_key("Active"));
        assert!(env.types.contains_key("Archived"));
    }
}
