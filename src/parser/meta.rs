use crate::ast::{Cardinality, ConstraintExpr, RawValue, Spanned};
use crate::meta::model::{Annotation, SourceAnnotation};
use crate::parser::{ident, padded, string_literal, type_ident, ws, ParserExtra, ParserInput};
use chumsky::prelude::*;

/// Parse a constraint expression for @constraint annotations
fn constraint_expr<'a>() -> impl Parser<'a, ParserInput<'a>, ConstraintExpr, ParserExtra<'a>> + Clone
{
    recursive(|expr| {
        // Parse: var.assoc(other)
        let assoc = ident()
            .then_ignore(just('.'))
            .then_ignore(text::keyword("assoc"))
            .then_ignore(just('('))
            .then_ignore(ws())
            .then(ident())
            .then_ignore(ws())
            .then_ignore(just(')'))
            .map(|(subject, target)| ConstraintExpr::Assoc { subject, target });

        // Parse: forall(field, var => body)
        let forall = text::keyword("forall")
            .ignore_then(just('('))
            .ignore_then(ws())
            .ignore_then(ident())
            .then_ignore(ws())
            .then_ignore(just(','))
            .then_ignore(ws())
            .then(ident())
            .then_ignore(ws())
            .then_ignore(just('='))
            .then_ignore(just('>'))
            .then_ignore(ws())
            .then(expr.clone())
            .then_ignore(ws())
            .then_ignore(just(')'))
            .map(|((field, var), body)| ConstraintExpr::ForAll {
                field,
                var,
                body: Box::new(body),
            });

        // Parse: exists(field, var => body)
        let exists = text::keyword("exists")
            .ignore_then(just('('))
            .ignore_then(ws())
            .ignore_then(ident())
            .then_ignore(ws())
            .then_ignore(just(','))
            .then_ignore(ws())
            .then(ident())
            .then_ignore(ws())
            .then_ignore(just('='))
            .then_ignore(just('>'))
            .then_ignore(ws())
            .then(expr.clone())
            .then_ignore(ws())
            .then_ignore(just(')'))
            .map(|((field, var), body)| ConstraintExpr::Exists {
                field,
                var,
                body: Box::new(body),
            });

        // Parse: !expr (unary not)
        let not = just('!')
            .ignore_then(ws())
            .ignore_then(expr.clone())
            .map(|e| ConstraintExpr::Not(Box::new(e)));

        // Parse parenthesized expr
        let parens = just('(')
            .ignore_then(ws())
            .ignore_then(expr.clone())
            .then_ignore(ws())
            .then_ignore(just(')'));

        // Atomic expressions
        let atomic = choice((forall, exists, assoc, not, parens));

        // Parse && chains (higher precedence)
        let and_chain = atomic
            .clone()
            .separated_by(padded(just("&&")))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut exprs| {
                if exprs.len() == 1 {
                    exprs.remove(0)
                } else {
                    exprs
                        .into_iter()
                        .reduce(|a, b| ConstraintExpr::And(Box::new(a), Box::new(b)))
                        .unwrap()
                }
            });

        // Parse || chains (lower precedence)
        and_chain
            .separated_by(padded(just("||")))
            .at_least(1)
            .collect::<Vec<_>>()
            .map(|mut exprs| {
                if exprs.len() == 1 {
                    exprs.remove(0)
                } else {
                    exprs
                        .into_iter()
                        .reduce(|a, b| ConstraintExpr::Or(Box::new(a), Box::new(b)))
                        .unwrap()
                }
            })
    })
}

pub fn annotation<'a>() -> impl Parser<'a, ParserInput<'a>, Annotation, ParserExtra<'a>> + Clone {
    let simple = just('@')
        .ignore_then(ident())
        .try_map(|s, span| match s.as_str() {
            "unique" => Ok(Annotation::Unique),
            "required" => Ok(Annotation::Required),
            _ => Err(Rich::custom(span, format!("unknown annotation: @{}", s))),
        });

    let bracket_list = ident()
        .separated_by(padded(just(',')))
        .collect::<Vec<_>>()
        .delimited_by(just('['), just(']'));

    let for_clause = ws()
        .ignore_then(text::keyword("for"))
        .ignore_then(ws())
        .ignore_then(bracket_list.clone())
        .or_not();

    let source = just('@')
        .ignore_then(text::keyword("source"))
        .ignore_then(ws())
        .ignore_then(bracket_list)
        .then(for_clause)
        .map(|(sources, for_paths)| {
            Annotation::Source(SourceAnnotation { sources, for_paths })
        });

    let assoc = just('@')
        .ignore_then(text::keyword("assoc"))
        .ignore_then(ws())
        .ignore_then(
            ident()
                .separated_by(padded(just(',')))
                .collect::<Vec<_>>()
                .delimited_by(just('['), just(']')),
        )
        .map(Annotation::Assoc);

    let constraint = just('@')
        .ignore_then(text::keyword("constraint"))
        .ignore_then(ws())
        .ignore_then(constraint_expr())
        .map(Annotation::Constraint);

    choice((source, assoc, constraint, simple))
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

        let type_val = type_ident().map(RawValue::Type);

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

        let cardinality = choice((
            just('*').to(Cardinality::ZeroOrMore),
            text::digits(10)
                .to_slice()
                .map(|s: &str| Cardinality::Exactly(s.parse().unwrap())),
        ));

        let wildcard_object = just('{')
            .ignore_then(ws())
            .ignore_then(cardinality.or_not())
            .then_ignore(ws())
            .then_ignore(just('_'))
            .then_ignore(ws())
            .then(value.clone())
            .then_ignore(ws())
            .then_ignore(just('}'))
            .map(|(card, v)| RawValue::WildcardObject {
                cardinality: card.unwrap_or(Cardinality::ZeroOrMore),
                value: Box::new(v),
            });

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

        let atomic = choice((
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
        .map_with(|v, e| Spanned::new(v, e.span().into_range()));

        let intersection = atomic
            .clone()
            .separated_by(padded(just('&')))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|mut v, e| {
                if v.len() == 1 {
                    v.remove(0)
                } else {
                    Spanned::new(RawValue::Intersection(v), e.span().into_range())
                }
            });

        intersection
            .separated_by(padded(just('|')))
            .at_least(1)
            .collect::<Vec<_>>()
            .map_with(|mut v, e| {
                if v.len() == 1 {
                    v.remove(0)
                } else {
                    Spanned::new(RawValue::Union(v), e.span().into_range())
                }
            })
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
pub struct MetaConstraint(pub ConstraintExpr);

#[derive(Debug, Clone, PartialEq)]
pub enum MetaItem {
    Field(MetaField),
    Block(MetaBlock),
    Constraint(MetaConstraint),
}

/// Parse standalone @constraint at block level
fn meta_constraint<'a>() -> impl Parser<'a, ParserInput<'a>, MetaConstraint, ParserExtra<'a>> + Clone
{
    just('@')
        .ignore_then(text::keyword("constraint"))
        .ignore_then(ws())
        .ignore_then(constraint_expr())
        .map(MetaConstraint)
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
            meta_constraint().map(MetaItem::Constraint),
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

// Top-level type definition: `@assoc [tag] event {* _ type} & {timestamp int}`
#[derive(Debug, Clone, PartialEq)]
pub struct MetaTypeDef {
    pub annotations: Vec<Annotation>,
    pub name: Spanned<String>,
    pub value: Spanned<RawValue>,
}

pub fn meta_type_def<'a>() -> impl Parser<'a, ParserInput<'a>, MetaTypeDef, ParserExtra<'a>> + Clone
{
    let annotations = annotation().separated_by(ws()).collect::<Vec<_>>();

    annotations
        .then_ignore(ws())
        .then(ident().map_with(|s, e| Spanned::new(s, e.span().into_range())))
        .then_ignore(ws())
        .then(meta_value())
        .map(|((annotations, name), value)| MetaTypeDef {
            annotations,
            name,
            value,
        })
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaTopLevel {
    Block(MetaBlock),
    TypeDef(MetaTypeDef),
}

impl MetaTopLevel {
    pub fn as_block(&self) -> Option<&MetaBlock> {
        match self {
            MetaTopLevel::Block(b) => Some(b),
            _ => None,
        }
    }
}

pub fn extract_blocks(items: &[MetaTopLevel]) -> Vec<MetaBlock> {
    items
        .iter()
        .filter_map(|i| i.as_block().cloned())
        .collect()
}

pub fn parse_meta(input: &str) -> Result<Vec<MetaTopLevel>, Vec<crate::error::ParseError>> {
    let top_level = choice((
        meta_block().map(MetaTopLevel::Block),
        meta_type_def().map(MetaTopLevel::TypeDef),
    ));

    let parser = ws()
        .ignore_then(top_level.separated_by(ws()).collect::<Vec<_>>())
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
            name String
            timestamp Int
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_extends() {
        let input = r#"userEvent extends event {
            userId String
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = extract_blocks(&result.unwrap());
        assert_eq!(blocks[0].extends.as_ref().unwrap().0, "event");
    }

    #[test]
    fn test_parse_annotations() {
        let input = r#"user {
            @unique id String
            @required name String
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = extract_blocks(&result.unwrap());
        if let MetaItem::Field(f) = &blocks[0].body[0].0 {
            assert!(f.annotations.contains(&Annotation::Unique));
        }
    }

    #[test]
    fn test_parse_wildcard_field() {
        let input = r#"metadata {
            _ String
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
        let blocks = extract_blocks(&result.unwrap());
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
            data {* _ String}
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_wildcard_object_exact() {
        let input = r#"config {
            data {1 _ String}
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_intersection() {
        let input = r#"event {
            data {* _ Type} & {timestamp Int}
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_union() {
        let input = r#"tag {
            value {1 _ Type} | String
        }"#;
        let result = parse_meta(input);
        assert!(result.is_ok());
    }
}
