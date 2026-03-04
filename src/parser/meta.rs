use crate::ast::*;
use crate::meta::model::Annotation;
use crate::parser::{ident, padded, string_literal, ws, ParserExtra, ParserInput};
use chumsky::prelude::*;

pub fn annotation<'a>() -> impl Parser<'a, ParserInput<'a>, Annotation, ParserExtra<'a>> + Clone {
    let simple = just('@')
        .ignore_then(ident())
        .try_map(|s, span| match s.as_str() {
            "unique" => Ok(Annotation::Unique),
            "required" => Ok(Annotation::Required),
            _ => Err(Rich::custom(span, format!("unknown annotation: @{}", s))),
        });

    let source = just('@')
        .ignore_then(text::keyword("source"))
        .ignore_then(ws())
        .ignore_then(
            ident()
                .separated_by(padded(just(',')))
                .collect::<Vec<_>>()
                .delimited_by(just('['), just(']')),
        )
        .map(Annotation::Source);

    choice((source, simple))
}

pub fn meta_value<'a>(
) -> impl Parser<'a, ParserInput<'a>, Spanned<RawValue>, ParserExtra<'a>> + Clone {
    recursive(|value| {
        let wildcard = just('_').to(RawValue::Wildcard);

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

        let type_val = ident().map(RawValue::Type);

        let list_type = just('[')
            .ignore_then(just(']'))
            .ignore_then(ident())
            .map(|t| RawValue::List(vec![Spanned::new(RawValue::Type(t), 0..0)]));

        let list_val = value
            .clone()
            .separated_by(padded(just(',')))
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(padded(just('[')), padded(just(']')))
            .map(RawValue::List);

        let wildcard_object = just('{')
            .ignore_then(ws())
            .ignore_then(just('_'))
            .ignore_then(ws())
            .ignore_then(value.clone())
            .then_ignore(ws())
            .then_ignore(just('}'))
            .map(|v| RawValue::Object(vec![(Spanned::new("_".to_string(), 0..0), v)]));

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
            wildcard,
            bool_val,
            string_val,
            float_val,
            int_val,
            list_type,
            list_val,
            wildcard_object,
            object_val,
            type_val,
        ))
        .map_with(|v, e| Spanned::new(v, e.span().into_range()))
    })
}

pub fn meta_field<'a>() -> impl Parser<'a, ParserInput<'a>, MetaField, ParserExtra<'a>> + Clone {
    let annotations = annotation().separated_by(ws()).collect::<Vec<_>>();

    let name_or_wildcard = choice((just('_').to("_".to_string()), ident()))
        .map_with(|s, e| Spanned::new(s, e.span().into_range()));

    let optional = just('?').or_not().map(|o| o.is_some());
    let generated = just('*').or_not().map(|g| g.is_some());

    annotations
        .then_ignore(ws())
        .then(name_or_wildcard)
        .then(optional)
        .then(generated)
        .then(ws().ignore_then(meta_value()))
        .map(
            |((((annotations, name), optional), generated), value)| MetaField {
                annotations,
                name,
                optional,
                generated,
                value,
            },
        )
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaField {
    pub annotations: Vec<Annotation>,
    pub name: Spanned<String>,
    pub optional: bool,
    pub generated: bool,
    pub value: Spanned<RawValue>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MetaBlock {
    pub kind: Spanned<String>,
    pub name: Option<Spanned<String>>,
    pub extends: Option<Spanned<String>>,
    pub body: Vec<Spanned<MetaItem>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaItem {
    Field(MetaField),
    Block(MetaBlock),
}

pub fn meta_block<'a>() -> impl Parser<'a, ParserInput<'a>, MetaBlock, ParserExtra<'a>> + Clone {
    recursive(|block| {
        let kind = ident().map_with(|s, e| Spanned::new(s, e.span().into_range()));
        let name = string_literal()
            .map_with(|s, e| Spanned::new(s, e.span().into_range()))
            .or_not();

        let extends = text::keyword("extends")
            .ignore_then(ws())
            .ignore_then(ident().map_with(|s, e| Spanned::new(s, e.span().into_range())))
            .or_not();

        let item = choice((
            block.map(MetaItem::Block),
            meta_field().map(MetaItem::Field),
        ))
        .map_with(|item, e| Spanned::new(item, e.span().into_range()));

        let body = item
            .separated_by(ws())
            .allow_trailing()
            .collect::<Vec<_>>()
            .delimited_by(padded(just('{')), padded(just('}')));

        kind.then_ignore(ws())
            .then(name)
            .then_ignore(ws())
            .then(extends)
            .then_ignore(ws())
            .then(body)
            .map(|(((kind, name), extends), body)| MetaBlock {
                kind,
                name,
                extends,
                body,
            })
    })
}

pub fn parse_meta(input: &str) -> Result<Vec<MetaBlock>, Vec<crate::error::ParseError>> {
    let parser = ws()
        .ignore_then(meta_block().separated_by(ws()).collect::<Vec<_>>())
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
    fn test_parse_meta_block() {
        let input = r#"event {
            name string
            timestamp int
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_extends() {
        let input = r#"userEvent extends event {
            userId string
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        assert_eq!(blocks[0].extends.as_ref().unwrap().0, "event");
    }

    #[test]
    fn test_parse_annotations() {
        let input = r#"user {
            @unique id string
            @required name string
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        if let MetaItem::Field(f) = &blocks[0].body[0].0 {
            assert!(f.annotations.contains(&Annotation::Unique));
        }
    }

    #[test]
    fn test_parse_wildcard_field() {
        let input = r#"metadata {
            _ string
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = result.unwrap();
        if let MetaItem::Field(f) = &blocks[0].body[0].0 {
            assert_eq!(f.name.0, "_");
        }
    }

    #[test]
    fn test_parse_list_type() {
        let input = r#"aggregate {
            events []event
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_wildcard_object() {
        let input = r#"config {
            data { _ string }
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }
}
