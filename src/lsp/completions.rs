use crate::ast::{Field, StructKind, TypeExpr};
use crate::span::S;
use crate::Compiler;
use std::path::Path;
use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

use super::diagnostics::position_to_offset;

const BASE_TYPES: &[&str] = &[
    "String",
    "Int",
    "Float",
    "Bool",
    "Uuid",
    "Date",
    "Timestamp",
    "Money",
];

#[derive(Debug)]
enum Context {
    TopLevel,
    AfterTypeName(String),
    AfterEquals,
    AfterColon,
    InStruct(Option<String>),
    Unknown,
}

pub fn complete(compiler: &Compiler, path: &Path, src: &str, pos: Position) -> Vec<CompletionItem> {
    let offset = position_to_offset(pos, src);
    let ctx = detect_context(src, offset);

    let env = compiler.get_env(path);
    let type_names: Vec<&str> = env
        .map(|e| e.types.keys().map(|s| s.as_str()).collect())
        .unwrap_or_default();

    match ctx {
        Context::TopLevel => top_level_completions(&type_names),
        Context::AfterTypeName(name) => after_type_name_completions(compiler, path, &name),
        Context::AfterEquals | Context::AfterColon => type_completions(&type_names),
        Context::InStruct(Some(type_name)) => {
            struct_field_completions(compiler, path, &type_name, &type_names)
        }
        Context::InStruct(None) => type_completions(&type_names),
        Context::Unknown => type_completions(&type_names),
    }
}

fn detect_context(src: &str, offset: usize) -> Context {
    let before = &src[..offset];

    // Find the last significant token
    let trimmed = before.trim_end();

    // Check for obvious triggers
    if trimmed.ends_with('=') {
        return Context::AfterEquals;
    }
    if trimmed.ends_with(':') {
        return Context::AfterColon;
    }

    // Check if we're in a struct context
    if let Some(brace_pos) = trimmed.rfind('{') {
        let after_brace = &trimmed[brace_pos + 1..];
        // Check if there's a closing brace between the opening brace and cursor
        if !after_brace.contains('}') {
            // We're inside a struct
            // Try to find what type this struct belongs to
            let before_brace = &trimmed[..brace_pos].trim_end();

            // Pattern: "name = TypeName {" for instances
            if let Some(eq_pos) = before_brace.rfind('=') {
                let after_eq = before_brace[eq_pos + 1..].trim();
                let type_name = after_eq.split_whitespace().next();
                if let Some(tn) = type_name {
                    if tn.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                        return Context::InStruct(Some(tn.to_string()));
                    }
                }
            }

            return Context::InStruct(None);
        }
    }

    // Check if we're at the start of a line (top-level)
    let last_line_start = before.rfind('\n').map(|p| p + 1).unwrap_or(0);
    let current_line = &before[last_line_start..];
    let trimmed_line = current_line.trim();

    if trimmed_line.is_empty() {
        return Context::TopLevel;
    }

    // Check if line starts with 'type'
    if trimmed_line.starts_with("type ") {
        let after_type = trimmed_line.strip_prefix("type ").unwrap().trim();
        // After "type Name = " we want type completions
        if after_type.contains('=') {
            return Context::AfterEquals;
        }
    }

    // If line starts with lowercase (could be instance name)
    if let Some(c) = trimmed_line.chars().next() {
        if c.is_lowercase() {
            // Pattern: "name = TypeName"
            if trimmed_line.contains('=') {
                let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
                if parts.len() == 2 {
                    let after_eq = parts[1].trim();
                    if let Some(type_name) = after_eq.split_whitespace().next() {
                        if type_name
                            .chars()
                            .next()
                            .map(|c| c.is_uppercase())
                            .unwrap_or(false)
                        {
                            return Context::AfterTypeName(type_name.to_string());
                        }
                    }
                    return Context::AfterEquals;
                }
            }
        }
    }

    Context::Unknown
}

fn top_level_completions(type_names: &[&str]) -> Vec<CompletionItem> {
    let mut items = vec![
        CompletionItem {
            label: "type".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            insert_text: Some("type ${1:Name} = ${2:Body}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            detail: Some("Define a new type".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "@main".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Mark instance as main entry".to_string()),
            ..Default::default()
        },
    ];

    // Add existing type names for instance creation
    for name in type_names {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Create instance".to_string()),
            ..Default::default()
        });
    }

    items
}

fn type_completions(type_names: &[&str]) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Base types
    for base in BASE_TYPES {
        items.push(CompletionItem {
            label: base.to_string(),
            kind: Some(CompletionItemKind::TYPE_PARAMETER),
            detail: Some("Built-in type".to_string()),
            ..Default::default()
        });
    }

    // User-defined types
    for name in type_names {
        items.push(CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("User type".to_string()),
            ..Default::default()
        });
    }

    // Struct snippet
    items.push(CompletionItem {
        label: "{...}".to_string(),
        kind: Some(CompletionItemKind::STRUCT),
        insert_text: Some("{${1}}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some("Struct".to_string()),
        ..Default::default()
    });

    // List snippet
    items.push(CompletionItem {
        label: "[]".to_string(),
        kind: Some(CompletionItemKind::STRUCT),
        insert_text: Some("[${1}]".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some("List".to_string()),
        ..Default::default()
    });

    items
}

fn after_type_name_completions(
    compiler: &Compiler,
    path: &Path,
    type_name: &str,
) -> Vec<CompletionItem> {
    let mut items = vec![CompletionItem {
        label: "{".to_string(),
        kind: Some(CompletionItemKind::STRUCT),
        insert_text: Some("{${1}}".to_string()),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        detail: Some("Struct body".to_string()),
        ..Default::default()
    }];

    // If we know the type, suggest its fields
    if let Some(env) = compiler.get_env(path) {
        if let Some(decl) = env.get_type(type_name) {
            if let Some(fields) = get_struct_fields(&decl.node.body) {
                let snippet = generate_struct_snippet(fields);
                items.insert(
                    0,
                    CompletionItem {
                        label: format!("{{ ... }} ({})", type_name),
                        kind: Some(CompletionItemKind::STRUCT),
                        insert_text: Some(snippet),
                        insert_text_format: Some(InsertTextFormat::SNIPPET),
                        detail: Some("Complete struct".to_string()),
                        ..Default::default()
                    },
                );
            }
        }
    }

    items
}

fn struct_field_completions(
    compiler: &Compiler,
    path: &Path,
    type_name: &str,
    all_types: &[&str],
) -> Vec<CompletionItem> {
    let mut items = Vec::new();

    // Get fields from type definition
    if let Some(env) = compiler.get_env(path) {
        if let Some(decl) = env.get_type(type_name) {
            if let Some(fields) = get_struct_fields(&decl.node.body) {
                for field in fields {
                    let type_str = type_expr_to_string(&field.node.ty.node);
                    items.push(CompletionItem {
                        label: field.node.name.node.clone(),
                        kind: Some(CompletionItemKind::FIELD),
                        insert_text: Some(format!("{} {}", field.node.name.node, type_str)),
                        detail: Some(format!("Field: {}", type_str)),
                        ..Default::default()
                    });
                }
            }
        }
    }

    // Also add type completions for field values
    items.extend(type_completions(all_types));

    items
}

fn get_struct_fields(ty: &S<TypeExpr>) -> Option<&[S<Field>]> {
    match &ty.node {
        TypeExpr::Struct(StructKind::Closed(fields)) => Some(fields),
        TypeExpr::Struct(StructKind::Open(fields)) => Some(fields),
        TypeExpr::Named(_) => None, // Would need to resolve
        _ => None,
    }
}

fn generate_struct_snippet(fields: &[S<Field>]) -> String {
    let mut parts = Vec::new();
    for (i, field) in fields.iter().enumerate() {
        let type_str = type_expr_to_string(&field.node.ty.node);
        parts.push(format!(
            "{} ${{{}:{}}}",
            field.node.name.node,
            i + 1,
            type_str
        ));
    }
    format!("{{{}}}", parts.join(", "))
}

fn type_expr_to_string(ty: &TypeExpr) -> String {
    match ty {
        TypeExpr::Base(b) => format!("{:?}", b),
        TypeExpr::Named(n) => n.clone(),
        TypeExpr::LitString(s) => format!("\"{}\"", s),
        TypeExpr::LitInt(i) => i.to_string(),
        TypeExpr::LitBool(b) => b.to_string(),
        TypeExpr::Reference(r) => format!("&{}", r),
        TypeExpr::List(_, inner) => format!("[{}]", type_expr_to_string(&inner.node)),
        TypeExpr::Struct(_) => "{...}".to_string(),
        TypeExpr::Union(_) => "|...".to_string(),
        TypeExpr::Intersection(_, _) => "&...".to_string(),
        TypeExpr::Concrete(inner) => format!("*{}", type_expr_to_string(&inner.node)),
        TypeExpr::RefinableRef(r) => format!("-{}", r),
    }
}
