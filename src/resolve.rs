use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticCode};
use crate::span::{Span, S};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct TypeEnv {
    pub metas: HashMap<String, S<MetaDecl>>,
    pub instances: HashMap<String, S<Instance>>,
    pub instance_files: HashMap<String, PathBuf>,
    pub main_instance: Option<String>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            metas: HashMap::new(),
            instances: HashMap::new(),
            instance_files: HashMap::new(),
            main_instance: None,
        }
    }

    pub fn get_meta(&self, name: &str) -> Option<&S<MetaDecl>> {
        self.metas.get(name)
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

pub fn resolve(file: &File, path: &Path) -> (TypeEnv, Vec<Diagnostic>) {
    resolve_with_imports(file, path, TypeEnv::new())
}

pub fn resolve_with_imports(
    file: &File,
    path: &Path,
    imported_env: TypeEnv,
) -> (TypeEnv, Vec<Diagnostic>) {
    let mut env = imported_env;
    let mut errors = Vec::new();

    // Collect all meta declarations
    for item in &file.items {
        if let Item::MetaDecl(decl) = &item.node {
            let name = &decl.name.node;
            if env.metas.contains_key(name) {
                errors.push(
                    Diagnostic::error(
                        decl.name.span.clone(),
                        format!("Duplicate meta: {}", name),
                        path,
                    )
                    .with_code(DiagnosticCode::DuplicateMeta),
                );
            } else {
                env.metas
                    .insert(name.clone(), S::new(decl.clone(), item.span.clone()));
            }
        }
    }

    // Auto-register implicit marker meta from union variants
    let mut implicit_metas: Vec<(String, Span)> = Vec::new();
    for item in &file.items {
        if let Item::MetaDecl(decl) = &item.node {
            collect_implicit_union_variants(&decl.body, &env, &mut implicit_metas);
        }
    }
    for (name, span) in implicit_metas {
        if !env.metas.contains_key(&name) && !is_base_type(&name) {
            let decl = MetaDecl {
                name: S::new(name.clone(), span.clone()),
                annotations: Vec::new(),
                body: S::new(
                    TypeExpr::Struct(StructKind::Closed(Vec::new())),
                    span.clone(),
                ),
            };
            env.metas.insert(name, S::new(decl, span));
        }
    }

    // Collect all instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            let name = &inst.name.node;
            if env.instances.contains_key(name) {
                errors.push(
                    Diagnostic::error(
                        inst.name.span.clone(),
                        format!("Duplicate instance: {}", name),
                        path,
                    )
                    .with_code(DiagnosticCode::DuplicateInstance),
                );
            } else {
                env.instances
                    .insert(name.clone(), S::new(inst.clone(), item.span.clone()));
                env.instance_files.insert(name.clone(), path.to_path_buf());
            }

            // Check for @main
            for ann in &inst.annotations {
                if matches!(ann.node, Annotation::Main) {
                    if env.main_instance.is_some() {
                        errors.push(
                            Diagnostic::error(ann.span.clone(), "Multiple @main annotations", path)
                                .with_code(DiagnosticCode::MultipleMain),
                        );
                    } else {
                        env.main_instance = Some(name.clone());
                    }
                }
            }
        }
    }

    // Check for unknown meta references in meta declarations
    for item in &file.items {
        if let Item::MetaDecl(decl) = &item.node {
            check_meta_refs(&decl.body, &env, path, &mut errors);
        }
    }

    // Check for unknown meta references in instances
    for item in &file.items {
        if let Item::Instance(inst) = &item.node {
            let type_name = &inst.type_name.node;
            if !env.metas.contains_key(type_name) && !is_base_type(type_name) {
                errors.push(
                    Diagnostic::error(
                        inst.type_name.span.clone(),
                        format!("Unknown type: {}", type_name),
                        path,
                    )
                    .with_code(DiagnosticCode::UnknownType),
                );
            }
        }
    }

    // Check for cycles in meta definitions
    check_cycles(&env, path, &mut errors);

    (env, errors)
}

fn is_base_type(name: &str) -> bool {
    BaseType::from_name(name).is_some()
}

/// Pre-order walk over a type expression tree. `f` is called on every node and
/// returns whether to descend into that node's children.
fn walk_type_expr(ty: &S<TypeExpr>, f: &mut impl FnMut(&S<TypeExpr>) -> bool) {
    if !f(ty) {
        return;
    }
    match &ty.node {
        TypeExpr::Concrete(inner) | TypeExpr::List(_, inner) => walk_type_expr(inner, f),
        TypeExpr::Union(variants) => {
            for v in variants {
                walk_type_expr(v, f);
            }
        }
        TypeExpr::Intersection(left, right) => {
            walk_type_expr(left, f);
            walk_type_expr(right, f);
        }
        TypeExpr::Struct(StructKind::Closed(fields) | StructKind::Open(fields)) => {
            for field in fields {
                walk_type_expr(&field.node.ty, f);
            }
        }
        TypeExpr::Struct(StructKind::Anonymous(types)) => {
            for ty in types.iter().flatten() {
                walk_type_expr(ty, f);
            }
        }
        _ => {}
    }
}

fn collect_implicit_union_variants(ty: &S<TypeExpr>, env: &TypeEnv, out: &mut Vec<(String, Span)>) {
    walk_type_expr(ty, &mut |node| {
        if let TypeExpr::Union(variants) = &node.node {
            for v in variants {
                if let TypeExpr::Named(name) = &v.node {
                    if !env.metas.contains_key(name) && !is_base_type(name) {
                        out.push((name.clone(), v.span.clone()));
                    }
                }
            }
        }
        true
    });
}

fn check_meta_refs(ty: &S<TypeExpr>, env: &TypeEnv, path: &Path, errors: &mut Vec<Diagnostic>) {
    walk_type_expr(ty, &mut |node| {
        match &node.node {
            TypeExpr::Named(name) if !env.metas.contains_key(name) => {
                errors.push(
                    Diagnostic::error(node.span.clone(), format!("Unknown type: {}", name), path)
                        .with_code(DiagnosticCode::UnknownType),
                );
            }
            TypeExpr::Reference(name) if !env.metas.contains_key(name) => {
                errors.push(
                    Diagnostic::error(
                        node.span.clone(),
                        format!("Unknown meta in reference: {}", name),
                        path,
                    )
                    .with_code(DiagnosticCode::UnknownMetaInReference),
                );
            }
            _ => {}
        }
        true
    });
}

fn check_cycles(env: &TypeEnv, path: &Path, errors: &mut Vec<Diagnostic>) {
    let mut visited = HashSet::new();
    let mut in_stack = HashSet::new();

    // Sorted so the DFS entry points — and thus the reported cycle edge — are
    // deterministic across runs.
    let mut names: Vec<&String> = env.metas.keys().collect();
    names.sort();
    for name in names {
        if !visited.contains(name.as_str()) {
            check_cycles_dfs(name, env, path, &mut visited, &mut in_stack, errors);
        }
    }
}

/// Returns true when a cycle was reported somewhere below `name`, so callers
/// stop exploring and a single cycle yields a single diagnostic.
fn check_cycles_dfs(
    name: &str,
    env: &TypeEnv,
    path: &Path,
    visited: &mut HashSet<String>,
    in_stack: &mut HashSet<String>,
    errors: &mut Vec<Diagnostic>,
) -> bool {
    visited.insert(name.to_string());
    in_stack.insert(name.to_string());

    let mut reported = false;
    if let Some(decl) = env.metas.get(name) {
        let deps = collect_direct_deps(&decl.node.body);
        for dep in deps {
            if in_stack.contains(&dep) {
                errors.push(
                    Diagnostic::error(
                        decl.node.name.span.clone(),
                        format!("Cyclic reference: {} -> {}", name, dep),
                        path,
                    )
                    .with_code(DiagnosticCode::CyclicReference),
                );
                reported = true;
                break;
            } else if !visited.contains(&dep)
                && check_cycles_dfs(&dep, env, path, visited, in_stack, errors)
            {
                reported = true;
                break;
            }
        }
    }

    in_stack.remove(name);
    reported
}

fn collect_direct_deps(ty: &S<TypeExpr>) -> Vec<String> {
    let mut deps = Vec::new();
    walk_type_expr(ty, &mut |node| match &node.node {
        TypeExpr::Named(name) | TypeExpr::Reference(name) => {
            deps.push(name.clone());
            true
        }
        // A list is not a direct containment: `meta A = []A` is well-founded
        // because the empty list terminates the recursion. Any cycle running
        // through a list element is therefore not an error, and cycles among
        // the element types themselves are found when DFS starts from them.
        TypeExpr::List(_, _) => false,
        _ => true,
    });
    deps.sort();
    deps.dedup();
    deps
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse;

    fn resolve_str(s: &str) -> (TypeEnv, Vec<Diagnostic>) {
        let file = parse(s, Path::new("test.ilk")).unwrap();
        resolve(&file, Path::new("test.ilk"))
    }

    #[test]
    fn test_type_collection() {
        let (env, errors) = resolve_str("meta Foo = {x Int}");
        assert!(errors.is_empty());
        assert!(env.metas.contains_key("Foo"));
    }

    #[test]
    fn test_instance_collection() {
        let (env, errors) = resolve_str("meta Foo = {x Int}\nfoo = Foo {x Int}");
        assert!(errors.is_empty());
        assert!(env.instances.contains_key("foo"));
    }

    #[test]
    fn test_main_instance() {
        let (env, errors) = resolve_str("meta Foo = {...}\n@main\nfoo = Foo {x Int}");
        assert!(errors.is_empty());
        assert_eq!(env.main_instance, Some("foo".to_string()));
    }

    #[test]
    fn test_forward_refs() {
        let (_env, errors) = resolve_str("meta A = B\nmeta B = {x Int}");
        assert!(errors.is_empty());
    }

    #[test]
    fn test_cycles() {
        let (_env, errs) = resolve_str("meta A = B\nmeta B = A");
        assert!(errs
            .iter()
            .any(|e| e.code == DiagnosticCode::CyclicReference));
    }

    #[test]
    fn test_cycle_reported_once() {
        // Two paths into the same cycle must not duplicate the diagnostic
        let (_env, errs) = resolve_str("meta A = B | B\nmeta B = A");
        let cyclic = errs
            .iter()
            .filter(|e| e.code == DiagnosticCode::CyclicReference)
            .count();
        assert_eq!(cyclic, 1, "{:?}", errs);
    }

    #[test]
    fn test_list_breaks_cycle() {
        // []A is well-founded: the empty list terminates the recursion
        let (_env, errs) = resolve_str("meta A = []A");
        assert!(errs.is_empty(), "{:?}", errs);
    }

    #[test]
    fn test_list_of_recursive_struct_ok() {
        let (_env, errs) = resolve_str("meta Node = {children []Node}");
        assert!(errs.is_empty(), "{:?}", errs);
    }

    #[test]
    fn test_multiple_main() {
        let (_env, errs) =
            resolve_str("meta A = {}\nmeta B = {}\n@main\na = A {}\n@main\nb = B {}");
        assert!(errs.iter().any(|e| e.code == DiagnosticCode::MultipleMain));
    }

    #[test]
    fn test_unknown_type() {
        let (_env, errs) = resolve_str("meta A = Unknown");
        assert!(errs.iter().any(|e| e.code == DiagnosticCode::UnknownType));
    }

    #[test]
    fn test_unknown_instance_type() {
        let (_env, errs) = resolve_str("foo = Unknown {x Int}");
        assert!(errs.iter().any(|e| e.code == DiagnosticCode::UnknownType));
    }

    #[test]
    fn test_implicit_union_marker_types() {
        let (env, errors) = resolve_str(
            "meta Status = Pending | Active | Archived\nmeta Process = { status! Status }",
        );
        assert!(errors.is_empty());
        assert!(env.metas.contains_key("Status"));
        assert!(env.metas.contains_key("Pending"));
        assert!(env.metas.contains_key("Active"));
        assert!(env.metas.contains_key("Archived"));
    }
}
