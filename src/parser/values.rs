use chumsky::prelude::*;

use crate::ast::*;
use crate::span::{Spanned, S};

use super::common::*;

fn lit_string_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| Value::LitString(s.to_string()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn lit_int_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    just('-')
        .or_not()
        .then(text::int(10))
        .to_slice()
        .map(|s: &str| Value::LitInt(s.parse().expect("valid int literal from parser")))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn lit_bool_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    choice((just("true").to(true), just("false").to(false)))
        .map(Value::LitBool)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn type_ref_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    choice((
        just("Uuid").to("Uuid"),
        just("String").to("String"),
        just("Int").to("Int"),
        just("Float").to("Float"),
        just("Bool").to("Bool"),
        just("Date").to("Date"),
        just("Timestamp").to("Timestamp"),
        just("Money").to("Money"),
    ))
    .map(|s| Value::TypeRef(s.to_string()))
    .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn dot_path<'a>() -> impl Parser<'a, ParserInput<'a>, Vec<String>, ParserExtra<'a>> + Clone {
    ident()
        .separated_by(just('.'))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|parts| parts.into_iter().map(|p| p.node).collect())
}

fn field_origin<'a>() -> impl Parser<'a, ParserInput<'a>, FieldOrigin, ParserExtra<'a>> + Clone {
    choice((
        just('*').to(FieldOrigin::Generated),
        just(" = compute(")
            .ignore_then(
                dot_path()
                    .separated_by(just(',').then(ws()))
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(')'))
            .map(FieldOrigin::Computed),
        just(" = ").ignore_then(dot_path()).map(FieldOrigin::Mapped),
        empty().to(FieldOrigin::None),
    ))
}

fn instance_field<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<InstanceField>, ParserExtra<'a>> + Clone {
    let doc = just("@doc")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .then_ignore(ws_nl())
        .or_not();

    doc.then(ident())
        .then(just('?').or_not().map(|o| o.is_some()))
        .then_ignore(ws())
        .then(value)
        .then(field_origin())
        .map(|((((doc, name), optional), value), origin)| InstanceField {
            name,
            optional,
            value,
            origin,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn instance_struct<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    just('{')
        .ignore_then(ws_nl())
        .ignore_then(
            instance_field(value)
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(Value::Struct)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn refinement_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    recursive(|refinement_val| {
        let simple_value = choice((
            type_ref_value(),
            lit_int_value(),
            lit_string_value(),
            lit_bool_value(),
        ));

        let refinement_struct = just('{')
            .ignore_then(ws_nl())
            .ignore_then(
                refinement_field_inner(refinement_val)
                    .separated_by(sep())
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(ws_nl())
            .then_ignore(just('}'))
            .map(Value::Struct)
            .map_with(|v, e| Spanned::from_simple(v, e.span()));

        choice((refinement_struct, simple_value))
    })
}

fn refinement_field_inner<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<InstanceField>, ParserExtra<'a>> + Clone {
    let doc = just("@doc")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .then_ignore(ws_nl())
        .or_not();

    doc.then(ident())
        .then(just('?').or_not().map(|o| o.is_some()))
        .then_ignore(ws())
        .then(value)
        .then(field_origin())
        .map(|((((doc, name), optional), value), origin)| InstanceField {
            name,
            optional,
            value,
            origin,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn refinement_field<'a>(
) -> impl Parser<'a, ParserInput<'a>, S<InstanceField>, ParserExtra<'a>> + Clone {
    refinement_field_inner(refinement_value())
}

fn list_element<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<ListElement>, ParserExtra<'a>> + Clone {
    let refinement = ident()
        .then_ignore(ws())
        .then_ignore(just('&'))
        .then_ignore(ws())
        .then_ignore(just('{'))
        .then_ignore(ws_nl())
        .then(
            refinement_field()
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(|(name, fields)| ListElement::Refinement(name.node, fields))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    let anon_struct = value
        .clone()
        .map(|v| ListElement::Value(v.node))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    let binding_ref = ident()
        .map(|n| ListElement::BindingRef(n.node))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    choice((refinement, anon_struct, binding_ref))
}

fn instance_list<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    just('[')
        .ignore_then(ws_nl())
        .ignore_then(
            list_element(value)
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just(']'))
        .map(Value::List)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn binding_ref_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    let refinement = text::ident()
        .filter(|s: &&str| {
            !matches!(
                *s,
                "Uuid"
                    | "String"
                    | "Int"
                    | "Float"
                    | "Bool"
                    | "Date"
                    | "Timestamp"
                    | "Money"
                    | "true"
                    | "false"
                    | "type"
                    | "import"
            )
        })
        .map(|s: &str| s.to_string())
        .then_ignore(ws())
        .then_ignore(just('&'))
        .then_ignore(ws())
        .then_ignore(just('{'))
        .then_ignore(ws_nl())
        .then(
            refinement_field()
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(|(name, fields)| Value::Refinement(name, fields))
        .map_with(|v, e| Spanned::from_simple(v, e.span()));

    let simple = text::ident()
        .filter(|s: &&str| {
            !matches!(
                *s,
                "Uuid"
                    | "String"
                    | "Int"
                    | "Float"
                    | "Bool"
                    | "Date"
                    | "Timestamp"
                    | "Money"
                    | "true"
                    | "false"
                    | "type"
                    | "import"
            )
        })
        .map(|s: &str| Value::BindingRef(s.to_string()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()));

    choice((refinement, simple))
}

pub fn value<'a>() -> impl Parser<'a, ParserInput<'a>, S<Value>, ParserExtra<'a>> + Clone {
    recursive(|value| {
        choice((
            lit_bool_value(),
            lit_string_value(),
            lit_int_value(),
            type_ref_value(),
            instance_struct(value.clone()),
            instance_list(value.clone()),
            // Variant: TypeName value
            ident()
                .then_ignore(ws())
                .then(value.clone())
                .map(|(name, body)| Value::Variant(name.node, Box::new(body)))
                .map_with(|v, e| Spanned::from_simple(v, e.span())),
            binding_ref_value(),
        ))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_value(s: &str) -> S<Value> {
        value().parse(s).into_result().unwrap()
    }

    // === Literals ===

    #[test]
    fn test_lit_string() {
        assert_eq!(parse_value("\"hello\"").node, Value::LitString("hello".into()));
    }

    #[test]
    fn test_lit_string_empty() {
        assert_eq!(parse_value("\"\"").node, Value::LitString("".into()));
    }

    #[test]
    fn test_lit_int_positive() {
        assert_eq!(parse_value("42").node, Value::LitInt(42));
    }

    #[test]
    fn test_lit_int_negative() {
        assert_eq!(parse_value("-3").node, Value::LitInt(-3));
    }

    #[test]
    fn test_lit_int_zero() {
        assert_eq!(parse_value("0").node, Value::LitInt(0));
    }

    #[test]
    fn test_lit_bool_true() {
        assert_eq!(parse_value("true").node, Value::LitBool(true));
    }

    #[test]
    fn test_lit_bool_false() {
        assert_eq!(parse_value("false").node, Value::LitBool(false));
    }

    // === Type refs ===

    #[test]
    fn test_all_type_refs() {
        let cases = ["String", "Int", "Float", "Bool", "Uuid", "Date", "Timestamp", "Money"];
        for input in cases {
            assert_eq!(parse_value(input).node, Value::TypeRef(input.into()));
        }
    }

    // === Binding ref ===

    #[test]
    fn test_binding_ref() {
        assert_eq!(parse_value("myBinding").node, Value::BindingRef("myBinding".into()));
    }

    #[test]
    fn test_binding_ref_not_type_ref() {
        // Known type names should parse as TypeRef, not BindingRef
        assert!(matches!(parse_value("String").node, Value::TypeRef(_)));
    }

    // === Struct ===

    #[test]
    fn test_struct_single_field() {
        let Value::Struct(fields) = parse_value("{x String}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].node.name.node, "x");
        assert_eq!(fields[0].node.value.node, Value::TypeRef("String".into()));
        assert!(!fields[0].node.optional);
    }

    #[test]
    fn test_struct_optional_field() {
        let Value::Struct(fields) = parse_value("{x? String}").node else {
            panic!("Expected struct");
        };
        assert!(fields[0].node.optional);
    }

    #[test]
    fn test_struct_multi_fields() {
        let Value::Struct(fields) = parse_value("{x Int, y \"hello\"}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].node.name.node, "x");
        assert_eq!(fields[1].node.name.node, "y");
        assert_eq!(fields[1].node.value.node, Value::LitString("hello".into()));
    }

    #[test]
    fn test_struct_newline_separated() {
        let Value::Struct(fields) = parse_value("{\n  x Int\n  y String\n}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_struct_trailing_comma() {
        let Value::Struct(fields) = parse_value("{x Int,}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_struct_nested() {
        let Value::Struct(fields) = parse_value("{inner {x Int}}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 1);
        assert!(matches!(fields[0].node.value.node, Value::Struct(_)));
    }

    #[test]
    fn test_struct_field_with_doc() {
        let Value::Struct(fields) = parse_value("{@doc \"help\"\nx Int}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields[0].node.doc, Some("help".into()));
    }

    // === Field origins ===

    #[test]
    fn test_field_origin_none() {
        let Value::Struct(fields) = parse_value("{x Int}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields[0].node.origin, FieldOrigin::None);
    }

    #[test]
    fn test_field_origin_generated() {
        let Value::Struct(fields) = parse_value("{x Int*}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields[0].node.origin, FieldOrigin::Generated);
    }

    #[test]
    fn test_field_origin_mapped() {
        let Value::Struct(fields) = parse_value("{x Int = foo.bar}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields[0].node.origin, FieldOrigin::Mapped(vec!["foo".into(), "bar".into()]));
    }

    #[test]
    fn test_field_origin_mapped_simple() {
        let Value::Struct(fields) = parse_value("{x Int = name}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields[0].node.origin, FieldOrigin::Mapped(vec!["name".into()]));
    }

    #[test]
    fn test_field_origin_computed() {
        let Value::Struct(fields) = parse_value("{x Int = compute(a.b, c)}").node else {
            panic!("Expected struct");
        };
        let FieldOrigin::Computed(deps) = &fields[0].node.origin else {
            panic!("Expected Computed");
        };
        assert_eq!(deps.len(), 2);
        assert_eq!(deps[0], vec!["a".to_string(), "b".to_string()]);
        assert_eq!(deps[1], vec!["c".to_string()]);
    }

    // === List ===

    #[test]
    fn test_list_empty() {
        let Value::List(elems) = parse_value("[]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 0);
    }

    #[test]
    fn test_list_binding_refs() {
        let Value::List(elems) = parse_value("[a, b, c]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 3);
        // Inside a list with full value parser, bare idents become Value(BindingRef)
        assert_eq!(elems[0].node, ListElement::Value(Value::BindingRef("a".into())));
        assert_eq!(elems[1].node, ListElement::Value(Value::BindingRef("b".into())));
        assert_eq!(elems[2].node, ListElement::Value(Value::BindingRef("c".into())));
    }

    #[test]
    fn test_list_with_values() {
        let Value::List(elems) = parse_value("[\"a\", 42, true]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 3);
        assert_eq!(elems[0].node, ListElement::Value(Value::LitString("a".into())));
        assert_eq!(elems[1].node, ListElement::Value(Value::LitInt(42)));
        assert_eq!(elems[2].node, ListElement::Value(Value::LitBool(true)));
    }

    #[test]
    fn test_list_newline_separated() {
        let Value::List(elems) = parse_value("[\n  a\n  b\n]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 2);
    }

    #[test]
    fn test_list_trailing_comma() {
        let Value::List(elems) = parse_value("[a,]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 1);
    }

    #[test]
    fn test_list_refinement_element() {
        let Value::List(elems) = parse_value("[foo & {x \"bar\"}]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 1);
        let ListElement::Refinement(name, fields) = &elems[0].node else {
            panic!("Expected refinement element");
        };
        assert_eq!(name, "foo");
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_list_mixed_elements() {
        let Value::List(elems) = parse_value("[a, b & {x Int}]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 2);
        assert!(matches!(elems[0].node, ListElement::Value(Value::BindingRef(_))));
        assert!(matches!(elems[1].node, ListElement::Refinement(_, _)));
    }

    // === Refinement ===

    #[test]
    fn test_refinement_single_field() {
        let Value::Refinement(name, fields) = parse_value("foo & {x \"hello\"}").node else {
            panic!("Expected refinement");
        };
        assert_eq!(name, "foo");
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].node.name.node, "x");
        assert_eq!(fields[0].node.value.node, Value::LitString("hello".into()));
    }

    #[test]
    fn test_refinement_multi_fields() {
        let Value::Refinement(_, fields) = parse_value("foo & {x Int, y \"bar\"}").node else {
            panic!("Expected refinement");
        };
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_refinement_nested_struct() {
        let Value::Refinement(_, fields) = parse_value("foo & {params {userId Uuid}}").node else {
            panic!("Expected refinement");
        };
        assert_eq!(fields.len(), 1);
        let Value::Struct(nested) = &fields[0].node.value.node else {
            panic!("Expected nested struct");
        };
        assert_eq!(nested.len(), 1);
    }

    #[test]
    fn test_refinement_with_type_ref_value() {
        let Value::Refinement(_, fields) = parse_value("foo & {x String}").node else {
            panic!("Expected refinement");
        };
        // inside refinement, String is a type ref value
        assert_eq!(fields[0].node.value.node, Value::TypeRef("String".into()));
    }

    #[test]
    fn test_refinement_with_bool() {
        let Value::Refinement(_, fields) = parse_value("foo & {active true}").node else {
            panic!("Expected refinement");
        };
        assert_eq!(fields[0].node.value.node, Value::LitBool(true));
    }

    // === Variant ===

    #[test]
    fn test_variant_with_struct() {
        let Value::Variant(name, body) = parse_value("Error {msg \"oops\"}").node else {
            panic!("Expected variant");
        };
        assert_eq!(name, "Error");
        assert!(matches!(body.node, Value::Struct(_)));
    }

    #[test]
    fn test_variant_with_string() {
        let Value::Variant(name, body) = parse_value("Tag \"hello\"").node else {
            panic!("Expected variant");
        };
        assert_eq!(name, "Tag");
        assert_eq!(body.node, Value::LitString("hello".into()));
    }

    #[test]
    fn test_variant_with_int() {
        let Value::Variant(name, body) = parse_value("Count 5").node else {
            panic!("Expected variant");
        };
        assert_eq!(name, "Count");
        assert_eq!(body.node, Value::LitInt(5));
    }

    // === Comments in values ===

    #[test]
    fn test_struct_with_comment() {
        let Value::Struct(fields) = parse_value("{\n// comment\nx Int\n}").node else {
            panic!("Expected struct");
        };
        assert_eq!(fields.len(), 1);
    }

    #[test]
    fn test_list_with_comment() {
        let Value::List(elems) = parse_value("[\n// comment\na\n]").node else {
            panic!("Expected list");
        };
        assert_eq!(elems.len(), 1);
    }
}
