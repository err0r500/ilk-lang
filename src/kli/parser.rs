use crate::error::Diagnostic;
use crate::kli::ast::*;
use crate::span::{Spanned, S};
use chumsky::prelude::*;
use std::path::Path;

type ParserInput<'a> = &'a str;
type ParserExtra<'a> = extra::Err<Rich<'a, char>>;

fn ident<'a>() -> impl Parser<'a, ParserInput<'a>, S<String>, ParserExtra<'a>> + Clone {
    text::ident()
        .map_with(|s: &str, e| Spanned::from_simple(s.to_string(), e.span()))
}

fn ws<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    one_of(" \t").repeated().ignored()
}

fn ws_nl<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    choice((
        one_of(" \t\n\r").ignored(),
        just("//")
            .then(none_of('\n').repeated())
            .then(just('\n').or_not())
            .ignored(),
    ))
    .repeated()
    .ignored()
}

fn sep<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    ws_nl()
        .then(just(',').or_not())
        .then(ws_nl())
        .ignored()
}

fn lit_string<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| KliValue::LitString(s.to_string()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn lit_int<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    just('-')
        .or_not()
        .then(text::int(10))
        .to_slice()
        .map(|s: &str| KliValue::LitInt(s.parse().unwrap()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn lit_bool<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    choice((just("true").to(true), just("false").to(false)))
        .map(KliValue::LitBool)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn type_ref<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
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
    .map(|s| KliValue::TypeRef(s.to_string()))
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
        // Generated: Type*
        just('*').to(FieldOrigin::Generated),
        // Computed: = compute(path, path, ...)
        just(" = compute(")
            .ignore_then(
                dot_path()
                    .separated_by(just(',').then(ws()))
                    .at_least(1)
                    .collect::<Vec<_>>(),
            )
            .then_ignore(just(')'))
            .map(FieldOrigin::Computed),
        // Mapped: = path.to.field
        just(" = ")
            .ignore_then(dot_path())
            .map(FieldOrigin::Mapped),
        // None
        empty().to(FieldOrigin::None),
    ))
}

fn kli_field<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<KliField>, ParserExtra<'a>> + Clone {
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
        .map(|((((doc, name), optional), value), origin)| KliField {
            name,
            optional,
            value,
            origin,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn kli_struct<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    just('{')
        .ignore_then(ws_nl())
        .ignore_then(
            kli_field(value)
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(KliValue::Struct)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn refinement_field<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliField>, ParserExtra<'a>> + Clone
{
    // In refinements, fields have simpler values (type refs only) with origins
    let simple_value = choice((type_ref(), lit_int(), lit_string(), lit_bool()));

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
        .then(simple_value)
        .then(field_origin())
        .map(|((((doc, name), optional), value), origin)| KliField {
            name,
            optional,
            value,
            origin,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn list_element<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<KliListElement>, ParserExtra<'a>> + Clone {
    // Refinement: binding & {fields}
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
        .map(|(name, fields)| KliListElement::Refinement(name.node, fields))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    // Anonymous struct
    let anon_struct = value
        .clone()
        .map(|v| KliListElement::Value(v.node))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    // Simple binding ref
    let binding_ref = ident()
        .map(|n| KliListElement::BindingRef(n.node))
        .map_with(|e, ex| Spanned::from_simple(e, ex.span()));

    choice((refinement, anon_struct, binding_ref))
}

fn kli_list<'a>(
    value: impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
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
        .map(KliValue::List)
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

fn binding_ref<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    text::ident()
        .filter(|s: &&str| {
            !matches!(
                *s,
                "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
                    | "true" | "false"
            )
        })
        .map(|s: &str| KliValue::BindingRef(s.to_string()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
}

pub fn kli_value<'a>() -> impl Parser<'a, ParserInput<'a>, S<KliValue>, ParserExtra<'a>> + Clone {
    recursive(|value| {
        choice((
            lit_bool(),
            lit_string(),
            lit_int(),
            type_ref(),
            kli_struct(value.clone()),
            kli_list(value.clone()),
            // Variant: TypeName value
            ident()
                .then_ignore(ws())
                .then(value.clone())
                .map(|(name, body)| KliValue::Variant(name.node, Box::new(body)))
                .map_with(|v, e| Spanned::from_simple(v, e.span())),
            binding_ref(),
        ))
    })
}

fn assocs<'a>() -> impl Parser<'a, ParserInput<'a>, Vec<S<String>>, ParserExtra<'a>> + Clone {
    just('<')
        .ignore_then(
            ident()
                .separated_by(just(',').then(ws()))
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .then_ignore(just('>'))
        .or_not()
        .map(|o| o.unwrap_or_default())
}

fn binding<'a>() -> impl Parser<'a, ParserInput<'a>, S<Binding>, ParserExtra<'a>> + Clone {
    let doc = just("@doc")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .then_ignore(ws_nl())
        .or_not();

    doc.then(ident())
        .then_ignore(ws())
        .then_ignore(just('='))
        .then_ignore(ws())
        .then(ident())
        .then(assocs())
        .then_ignore(ws())
        .then(kli_value())
        .map(|((((doc, name), type_name), assocs), body)| Binding {
            name,
            type_name,
            assocs,
            body,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|b, e| Spanned::from_simple(b, e.span()))
}

pub fn kli_file<'a>() -> impl Parser<'a, ParserInput<'a>, KliFile, ParserExtra<'a>> {
    ws_nl()
        .ignore_then(
            binding()
                .separated_by(ws_nl())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .map(|bindings| KliFile { bindings })
}

pub fn parse_kli(src: &str, file: &Path) -> Result<KliFile, Vec<Diagnostic>> {
    kli_file().parse(src).into_result().map_err(|errs| {
        errs.into_iter()
            .map(|e| Diagnostic::error(e.span().into_range(), e.to_string(), file.to_path_buf()))
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> S<KliValue> {
        kli_value().parse(s).into_result().unwrap()
    }

    // Phase 4.1: Basic values
    #[test]
    fn test_type_ref() {
        assert!(matches!(parse("String").node, KliValue::TypeRef(s) if s == "String"));
        assert!(matches!(parse("Int").node, KliValue::TypeRef(s) if s == "Int"));
    }

    #[test]
    fn test_literals() {
        assert!(matches!(parse("\"hello\"").node, KliValue::LitString(s) if s == "hello"));
        assert!(matches!(parse("42").node, KliValue::LitInt(42)));
        assert!(matches!(parse("true").node, KliValue::LitBool(true)));
    }

    #[test]
    fn test_binding_ref() {
        assert!(matches!(parse("myBinding").node, KliValue::BindingRef(s) if s == "myBinding"));
    }

    // Phase 4.2: Structs
    #[test]
    fn test_struct() {
        let v = parse("{x String}");
        if let KliValue::Struct(fields) = v.node {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].node.name.node, "x");
        } else {
            panic!("Expected struct");
        }
    }

    #[test]
    fn test_struct_multiline() {
        let v = parse("{\n  x String\n  y Int\n}");
        if let KliValue::Struct(fields) = v.node {
            assert_eq!(fields.len(), 2);
        } else {
            panic!("Expected struct");
        }
    }

    // Phase 4.3: Field origins
    #[test]
    fn test_field_generated() {
        let v = parse("{timestamp Int*}");
        if let KliValue::Struct(fields) = v.node {
            assert!(matches!(fields[0].node.origin, FieldOrigin::Generated));
        } else {
            panic!("Expected struct");
        }
    }

    #[test]
    fn test_field_mapped() {
        let v = parse("{x Int = fields.id}");
        if let KliValue::Struct(fields) = v.node {
            assert!(matches!(&fields[0].node.origin, FieldOrigin::Mapped(p) if p == &["fields", "id"]));
        } else {
            panic!("Expected struct");
        }
    }

    #[test]
    fn test_field_computed() {
        let v = parse("{x Int = compute(a, b)}");
        if let KliValue::Struct(fields) = v.node {
            if let FieldOrigin::Computed(paths) = &fields[0].node.origin {
                assert_eq!(paths.len(), 2);
            } else {
                panic!("Expected computed origin");
            }
        } else {
            panic!("Expected struct");
        }
    }

    // Phase 4.4: Optional fields
    #[test]
    fn test_optional_field() {
        let v = parse("{email? String}");
        if let KliValue::Struct(fields) = v.node {
            assert!(fields[0].node.optional);
        } else {
            panic!("Expected struct");
        }
    }

    // Phase 4.5: Lists
    #[test]
    fn test_list() {
        let v = parse("[a, b]");
        if let KliValue::List(elements) = v.node {
            assert_eq!(elements.len(), 2);
        } else {
            panic!("Expected list");
        }
    }

    #[test]
    fn test_list_refinement() {
        let v = parse("[a & {x Int*}]");
        if let KliValue::List(elements) = v.node {
            assert!(matches!(&elements[0].node, KliListElement::Refinement(name, _) if name == "a"));
        } else {
            panic!("Expected list");
        }
    }

    // Phase 4.6: Full kli file
    #[test]
    fn test_binding() {
        let src = "foo = Type {x Int}";
        let file = kli_file().parse(src).into_result().unwrap();
        assert_eq!(file.bindings.len(), 1);
        assert_eq!(file.bindings[0].node.name.node, "foo");
        assert_eq!(file.bindings[0].node.type_name.node, "Type");
    }

    #[test]
    fn test_binding_with_assocs() {
        let src = "foo = Type<a, b> {x Int}";
        let file = kli_file().parse(src).into_result().unwrap();
        assert_eq!(file.bindings[0].node.assocs.len(), 2);
    }

    #[test]
    fn test_parse_dcb_board_instance() {
        let src = std::fs::read_to_string("examples/dcb-board-instance-valid.kli").unwrap();
        let result = parse_kli(&src, Path::new("examples/dcb-board-instance-valid.kli"));
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        let file = result.unwrap();
        assert!(!file.bindings.is_empty());
    }
}
