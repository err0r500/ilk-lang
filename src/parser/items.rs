use chumsky::prelude::*;

use crate::ast::*;
use crate::span::{Spanned, S};

use super::common::*;
use super::types::{annotation, type_expr};
use super::values::value;

fn meta_decl<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    annotation()
        .then_ignore(ws_nl())
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(just("meta"))
        .then_ignore(ws())
        .then(ident())
        .then_ignore(ws())
        .then_ignore(just('='))
        .then_ignore(ws())
        .then(type_expr())
        .map(|((annotations, name), body)| {
            Item::MetaDecl(MetaDecl {
                name,
                annotations,
                body,
            })
        })
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

fn instance<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    let doc = just("@doc")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .then_ignore(ws_nl())
        .or_not();

    let main_ann = just("@main")
        .then_ignore(ws_nl())
        .or_not()
        .map(|o| o.is_some());

    main_ann
        .then(doc)
        .then(ident())
        .then_ignore(ws())
        .then_ignore(just('='))
        .then_ignore(ws())
        .then(ident())
        .then_ignore(ws())
        .then(value())
        .map(|((((is_main, doc), name), type_name), body)| {
            let annotations = if is_main {
                vec![Spanned::new(Annotation::Main, 0..0)]
            } else {
                vec![]
            };
            Item::Instance(Instance {
                name,
                type_name,
                body,
                annotations,
                doc: doc.map(|s: &str| s.to_string()),
            })
        })
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

fn import<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    just("import")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map_with(|s: &str, e| Spanned::from_simple(s.to_string(), e.span()))
        .then(
            ws().ignore_then(just("as"))
                .ignore_then(ws())
                .ignore_then(ident())
                .or_not(),
        )
        .map(|(path, alias)| Item::Import(Import { path, alias }))
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

fn item<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    choice((import(), meta_decl(), instance()))
}

pub(super) fn file<'a>() -> impl Parser<'a, ParserInput<'a>, File, ParserExtra<'a>> {
    ws_nl()
        .ignore_then(
            item()
                .separated_by(ws_nl())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .map(|items| File {
            items,
            comments: vec![],
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_file(s: &str) -> File {
        file().parse(s).into_result().unwrap()
    }

    fn parse_file_err(s: &str) {
        assert!(file().parse(s).into_result().is_err());
    }

    // === Meta declarations ===

    #[test]
    fn test_meta_decl_simple() {
        let f = parse_file("meta Foo = {x Int}");
        let td = f.meta_decls().next().unwrap();
        assert_eq!(td.name.node, "Foo");
        assert!(matches!(
            td.body.node,
            TypeExpr::Struct(StructKind::Closed(_))
        ));
    }

    #[test]
    fn test_meta_decl_base() {
        let f = parse_file("meta Name = String");
        let td = f.meta_decls().next().unwrap();
        assert_eq!(td.name.node, "Name");
        assert_eq!(td.body.node, TypeExpr::Base(BaseType::String));
    }

    #[test]
    fn test_meta_decl_union() {
        let f = parse_file("meta Status = \"active\" | \"inactive\"");
        let td = f.meta_decls().next().unwrap();
        assert!(matches!(td.body.node, TypeExpr::Union(_)));
    }

    #[test]
    fn test_meta_decl_list() {
        let f = parse_file("meta Names = []String");
        let td = f.meta_decls().next().unwrap();
        assert!(matches!(td.body.node, TypeExpr::List(_, _)));
    }

    #[test]
    fn test_meta_decl_reference() {
        let f = parse_file("meta FooRef = &Foo");
        let td = f.meta_decls().next().unwrap();
        assert_eq!(td.body.node, TypeExpr::Reference("Foo".into()));
    }

    #[test]
    fn test_meta_decl_intersection() {
        let f = parse_file("meta Ext = {...} & {id Uuid}");
        let td = f.meta_decls().next().unwrap();
        assert!(matches!(td.body.node, TypeExpr::Intersection(_, _)));
    }

    #[test]
    fn test_meta_decl_with_annotation() {
        let f = parse_file("@doc \"A doc\"\nmeta Foo = Int");
        let td = f.meta_decls().next().unwrap();
        assert_eq!(td.annotations.len(), 1);
        assert_eq!(td.annotations[0].node, Annotation::Doc("A doc".into()));
    }

    #[test]
    fn test_meta_decl_with_multiple_annotations() {
        let f = parse_file("@doc \"help\"\n@constraint true\nmeta Foo = {x Int}");
        let td = f.meta_decls().next().unwrap();
        assert_eq!(td.annotations.len(), 2);
    }

    #[test]
    fn test_meta_decl_no_annotations() {
        let f = parse_file("meta Foo = Int");
        let td = f.meta_decls().next().unwrap();
        assert!(td.annotations.is_empty());
    }

    // === Instances ===

    #[test]
    fn test_instance_simple() {
        let f = parse_file("foo = Foo {x Int}");
        let inst = f.instances().next().unwrap();
        assert_eq!(inst.name.node, "foo");
        assert_eq!(inst.type_name.node, "Foo");
        assert!(matches!(inst.body.node, Value::Struct(_)));
    }

    #[test]
    fn test_instance_with_list_body() {
        let f = parse_file("items = Items [a, b]");
        let inst = f.instances().next().unwrap();
        assert!(matches!(inst.body.node, Value::List(_)));
    }

    #[test]
    fn test_instance_with_binding_ref_body() {
        let f = parse_file("foo = Foo bar");
        let inst = f.instances().next().unwrap();
        assert_eq!(inst.body.node, Value::BindingRef("bar".into()));
    }

    #[test]
    fn test_instance_main_annotation() {
        let f = parse_file("@main\nboard = Board {x Int}");
        let inst = f.instances().next().unwrap();
        assert!(inst
            .annotations
            .iter()
            .any(|a| matches!(a.node, Annotation::Main)));
    }

    #[test]
    fn test_instance_not_main() {
        let f = parse_file("foo = Foo {x Int}");
        let inst = f.instances().next().unwrap();
        assert!(inst.annotations.is_empty());
    }

    #[test]
    fn test_instance_with_doc() {
        let f = parse_file("@doc \"my doc\"\nfoo = Foo {x Int}");
        let inst = f.instances().next().unwrap();
        assert_eq!(inst.doc, Some("my doc".into()));
    }

    #[test]
    fn test_instance_no_doc() {
        let f = parse_file("foo = Foo {x Int}");
        let inst = f.instances().next().unwrap();
        assert_eq!(inst.doc, None);
    }

    #[test]
    fn test_instance_main_and_doc() {
        let f = parse_file("@main\n@doc \"main board\"\nboard = Board {x Int}");
        let inst = f.instances().next().unwrap();
        assert!(inst
            .annotations
            .iter()
            .any(|a| matches!(a.node, Annotation::Main)));
        assert_eq!(inst.doc, Some("main board".into()));
    }

    // === Imports ===

    #[test]
    fn test_import_simple() {
        let f = parse_file("import \"./base.ilk\"");
        let imp = f.imports().next().unwrap();
        assert_eq!(imp.path.node, "./base.ilk");
        assert!(imp.alias.is_none());
    }

    #[test]
    fn test_import_with_alias() {
        let f = parse_file("import \"./base.ilk\" as base");
        let imp = f.imports().next().unwrap();
        assert_eq!(imp.path.node, "./base.ilk");
        assert_eq!(imp.alias.as_ref().unwrap().node, "base");
    }

    #[test]
    fn test_import_nested_path() {
        let f = parse_file("import \"../shared/types.ilk\"");
        let imp = f.imports().next().unwrap();
        assert_eq!(imp.path.node, "../shared/types.ilk");
    }

    // === File-level composition ===

    #[test]
    fn test_empty_file() {
        let f = parse_file("");
        assert_eq!(f.items.len(), 0);
    }

    #[test]
    fn test_whitespace_only_file() {
        let f = parse_file("  \n\n  ");
        assert_eq!(f.items.len(), 0);
    }

    #[test]
    fn test_single_type() {
        let f = parse_file("meta Foo = Int");
        assert_eq!(f.meta_decls().count(), 1);
        assert_eq!(f.instances().count(), 0);
        assert_eq!(f.imports().count(), 0);
    }

    #[test]
    fn test_single_instance() {
        let f = parse_file("foo = Foo {x Int}");
        assert_eq!(f.meta_decls().count(), 0);
        assert_eq!(f.instances().count(), 1);
        assert_eq!(f.imports().count(), 0);
    }

    #[test]
    fn test_imports_and_types() {
        let f = parse_file("import \"./a.ilk\"\n\nmeta Foo = Int");
        assert_eq!(f.imports().count(), 1);
        assert_eq!(f.meta_decls().count(), 1);
    }

    #[test]
    fn test_multiple_imports() {
        let f = parse_file("import \"./a.ilk\"\nimport \"./b.ilk\"");
        assert_eq!(f.imports().count(), 2);
    }

    #[test]
    fn test_multiple_types() {
        let f = parse_file("meta A = Int\nmeta B = String");
        assert_eq!(f.meta_decls().count(), 2);
    }

    #[test]
    fn test_multiple_instances() {
        let f = parse_file("a = A {x Int}\nb = B {y String}");
        assert_eq!(f.instances().count(), 2);
    }

    #[test]
    fn test_full_file() {
        let src = r#"
meta Tag = {_ String}

meta Event = {...} & {timestamp Int}

tag1 = Tag {x String}

ev = Event {
    id String
}

@main
board = Board {
    events [ev]
}
"#;
        let f = parse_file(src);
        assert_eq!(f.meta_decls().count(), 2);
        assert_eq!(f.instances().count(), 3);
    }

    #[test]
    fn test_file_with_comments() {
        let src = "// header comment\nmeta Foo = Int\n// another\nfoo = Foo 42";
        let f = parse_file(src);
        assert_eq!(f.meta_decls().count(), 1);
        assert_eq!(f.instances().count(), 1);
    }

    #[test]
    fn test_file_mixed_order() {
        let src = r#"
import "./base.ilk"
meta Foo = Int
foo = Foo 42
meta Bar = String
bar = Bar "hello"
"#;
        let f = parse_file(src);
        assert_eq!(f.imports().count(), 1);
        assert_eq!(f.meta_decls().count(), 2);
        assert_eq!(f.instances().count(), 2);
    }

    #[test]
    fn test_complex_instance_body() {
        let src = r#"
ev = Event {
    id Uuid
    name "click"
    params {
        userId Uuid
        page "home"
    }
    tags ["a", "b"]
}
"#;
        let f = parse_file(src);
        let inst = f.instances().next().unwrap();
        let Value::Struct(fields) = &inst.body.node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 4);
        assert_eq!(fields[0].node.name.node, "id");
        assert_eq!(fields[1].node.name.node, "name");
        assert!(matches!(fields[2].node.value.node, Value::Struct(_)));
        assert!(matches!(fields[3].node.value.node, Value::List(_)));
    }

    #[test]
    fn test_instance_with_refinements_in_list() {
        let src = r#"
board = Board {
    events [
        base & {name "click"}
        base & {name "view"}
    ]
}
"#;
        let f = parse_file(src);
        let inst = f.instances().next().unwrap();
        let Value::Struct(fields) = &inst.body.node else {
            panic!("Expected struct");
        };
        let Value::List(elems) = &fields[0].node.value.node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0].node, ListElement::Refinement(_, _)));
        assert!(matches!(elems[1].node, ListElement::Refinement(_, _)));
    }

    // === Error cases ===

    #[test]
    fn test_reject_bare_number() {
        // A bare number is not a valid item
        parse_file_err("42");
    }

    #[test]
    fn test_reject_type_without_body() {
        parse_file_err("meta Foo =");
    }

    #[test]
    fn test_reject_instance_without_type() {
        parse_file_err("foo = {x Int}");
    }
}
