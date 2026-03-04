use crate::ast::*;
use crate::parser::{ident, padded, string_literal, ws, ParserExtra, ParserInput};
use chumsky::prelude::*;

pub fn value<'a>() -> impl Parser<'a, ParserInput<'a>, Spanned<RawValue>, ParserExtra<'a>> + Clone {
    recursive(|value| {
        let bool_val = choice((
            text::keyword("true").to(true),
            text::keyword("false").to(false),
        ))
        .map(RawValue::Bool);

        let string_val = string_literal().map(RawValue::String);

        let float_val = just('-')
            .or_not()
            .then(text::digits(10))
            .then(just('.'))
            .then(text::digits(10))
            .to_slice()
            .map(|s: &str| RawValue::Float(s.parse().unwrap()));

        let int_val = just('-')
            .or_not()
            .then(text::digits(10))
            .to_slice()
            .map(|s: &str| RawValue::Int(s.parse().unwrap()));

        // TypeRefinement: Ident { field: type*, ... }
        // Override values support type with optional * suffix for generated
        let refinement_value = ident()
            .separated_by(just('.'))
            .at_least(1)
            .collect::<Vec<_>>()
            .then(just('*').or_not())
            .map_with(|(parts, star), e| {
                let mut val = RawValue::Ref(parts);
                if star.is_some() {
                    // Wrap in Object with __generated marker
                    val = RawValue::Object(vec![
                        (
                            Spanned::new("__type".to_string(), e.span().into_range()),
                            Spanned::new(val, e.span().into_range()),
                        ),
                        (
                            Spanned::new("__generated".to_string(), e.span().into_range()),
                            Spanned::new(RawValue::Bool(true), e.span().into_range()),
                        ),
                    ]);
                }
                Spanned::new(val, e.span().into_range())
            });

        let refinement_field = ident()
            .map_with(|s, e| Spanned::new(s, e.span().into_range()))
            .then_ignore(ws())
            .then(refinement_value);

        let type_refinement = ident()
            .then_ignore(ws())
            .then(
                refinement_field
                    .separated_by(padded(just(',')))
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(padded(just('{')), padded(just('}'))),
            )
            .map(|(base, overrides)| RawValue::TypeRefinement(TypeRefinement { base, overrides }));

        let ref_val = ident()
            .separated_by(just('.'))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(RawValue::Ref);

        let list_val = value
            .clone()
            .separated_by(padded(just(',')))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(padded(just('[')), padded(just(']')))
            .map(RawValue::List);

        let object_field = ident()
            .map_with(|s, e| Spanned::new(s, e.span().into_range()))
            .then_ignore(ws())
            .then(value.clone());

        let object_val = object_field
            .separated_by(padded(just(',')))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(padded(just('{')), padded(just('}')))
            .map(RawValue::Object);

        choice((
            bool_val,
            string_val,
            float_val,
            int_val,
            list_val,
            object_val,
            type_refinement,
            ref_val,
        ))
        .map_with(|v, e| Spanned::new(v, e.span().into_range()))
    })
}

pub fn field<'a>() -> impl Parser<'a, ParserInput<'a>, RawField, ParserExtra<'a>> + Clone {
    let name = ident().map_with(|s, e| Spanned::new(s, e.span().into_range()));
    let optional = just('?').or_not().map(|o| o.is_some());
    let generated = just('*').or_not().map(|g| g.is_some());

    name.then(optional)
        .then(generated)
        .then(ws().ignore_then(value()))
        .map(|(((name, optional), generated), value)| RawField {
            name,
            optional,
            generated,
            value,
        })
}

pub fn block<'a>() -> impl Parser<'a, ParserInput<'a>, RawBlock, ParserExtra<'a>> + Clone {
    recursive(|block| {
        let kind = ident().map_with(|s, e| Spanned::new(s, e.span().into_range()));
        let name = choice((
            ident().map_with(|s, e| Spanned::new(s, e.span().into_range())),
            string_literal().map_with(|s, e| Spanned::new(s, e.span().into_range())),
        ))
        .or_not();

        let item = choice((block.map(RawItem::Block), field().map(RawItem::Field)))
            .map_with(|item, e| Spanned::new(item, e.span().into_range()));

        let body = item
            .separated_by(ws())
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(padded(just('{')), padded(just('}')));

        kind.then_ignore(ws())
            .then(name)
            .then_ignore(ws())
            .then(body)
            .map(|((kind, name), body)| RawBlock { kind, name, body })
    })
}

pub fn parse_instance(input: &str) -> Result<Vec<RawBlock>, Vec<crate::error::ParseError>> {
    let parser = ws()
        .ignore_then(block().separated_by(ws()).collect::<Vec<_>>())
        .then_ignore(ws())
        .then_ignore(end());

    parser
        .parse(input)
        .into_result()
        .map_err(crate::error::convert_errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_block() {
        let input = r#"event "hello" {
            name "HelloEvent"
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        assert_eq!(blocks.len(), 1);
        assert_eq!(blocks[0].kind.0, "event");
        assert_eq!(blocks[0].name.as_ref().unwrap().0, "hello");
    }

    #[test]
    fn test_parse_nested_block() {
        let input = r#"aggregate "user" {
            event "created" {
                userId string
            }
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok(), "parse failed: {:?}", result.err());
    }

    #[test]
    fn test_parse_list_value() {
        let input = r#"config {
            items [1, 2, 3]
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_object_value() {
        let input = r#"config {
            meta { key "value", count 42 }
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_ref_value() {
        let input = r#"command {
            target user.created
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        if let RawItem::Field(f) = &blocks[0].body[0].0 {
            assert!(matches!(&f.value.0, RawValue::Ref(parts) if parts == &["user", "created"]));
        }
    }

    #[test]
    fn test_parse_optional_generated() {
        let input = r#"event {
            id?* string
        }"#;
        let result = parse_instance(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        if let RawItem::Field(f) = &blocks[0].body[0].0 {
            assert!(f.optional);
            assert!(f.generated);
        }
    }
}
