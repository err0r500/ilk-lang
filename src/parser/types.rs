use chumsky::prelude::*;

use crate::ast::*;
use crate::span::{Spanned, S};

use super::common::*;

fn base_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    choice((
        just("*").to(BaseType::Wildcard),
        just("Uuid").to(BaseType::Uuid),
        just("String").to(BaseType::String),
        just("Int").to(BaseType::Int),
        just("Float").to(BaseType::Float),
        just("Bool").to(BaseType::Bool),
        just("Date").to(BaseType::Date),
        just("Timestamp").to(BaseType::Timestamp),
        just("Money").to(BaseType::Money),
    ))
    .map(TypeExpr::Base)
    .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn lit_string_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone
{
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| TypeExpr::LitString(s.to_string()))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn lit_int_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('-')
        .or_not()
        .then(text::int(10))
        .to_slice()
        .map(|s: &str| TypeExpr::LitInt(s.parse().expect("valid int literal from parser")))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn lit_bool_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    choice((just("true").to(true), just("false").to(false)))
        .map(TypeExpr::LitBool)
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn concrete<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just("Concrete<")
        .ignore_then(type_expr)
        .then_ignore(just('>'))
        .map(|t| TypeExpr::Concrete(Box::new(t)))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn cardinality<'a>() -> impl Parser<'a, ParserInput<'a>, Cardinality, ParserExtra<'a>> + Clone {
    let num = text::int(10).map(|s: &str| s.parse::<usize>().expect("valid usize from parser"));

    choice((
        num.clone()
            .then_ignore(just(".."))
            .then(num.clone())
            .map(|(n, m)| Cardinality::Range(n, m)),
        num.clone()
            .then_ignore(just(".."))
            .map(Cardinality::AtLeast),
        just("..").ignore_then(num.clone()).map(Cardinality::AtMost),
        num.map(Cardinality::Exact),
        empty().to(Cardinality::Any),
    ))
}

fn list_type<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('[')
        .ignore_then(cardinality())
        .then_ignore(just(']'))
        .then(type_expr)
        .map(|(c, t)| TypeExpr::List(c, Box::new(t)))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn reference<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('&')
        .ignore_then(ident())
        .map(|n| TypeExpr::Reference(n.node))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn named_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    text::ident()
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
                    | "Concrete"
                    | "all"
                    | "exists"
                    | "unique"
                    | "count"
                    | "templateVars"
                    | "keys"
                    | "in"
                    | "type"
                    | "import"
            )
        })
        .map(|s: &str| TypeExpr::Named(s.to_string()))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn refinable_ref<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('-')
        .ignore_then(text::ident())
        .map(|s: &str| TypeExpr::RefinableRef(s.to_string()))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn source_path<'a>() -> impl Parser<'a, ParserInput<'a>, S<SourcePath>, ParserExtra<'a>> + Clone {
    ident()
        .separated_by(just('.'))
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|parts| {
            if parts.len() == 1 {
                SourcePath::Simple(parts[0].node.clone())
            } else {
                SourcePath::Dotted(parts.into_iter().map(|p| p.node).collect())
            }
        })
        .map_with(|p, e| Spanned::from_simple(p, e.span()))
}

fn constraint_expr<'a>(
) -> impl Parser<'a, ParserInput<'a>, S<ConstraintExpr>, ParserExtra<'a>> + Clone {
    recursive(|expr| {
        let atom = choice((
            just("true")
                .to(ConstraintExpr::Bool(true))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("false")
                .to(ConstraintExpr::Bool(false))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just('-')
                .or_not()
                .then(text::int(10))
                .to_slice()
                .map(|s: &str| {
                    ConstraintExpr::Int(s.parse().expect("valid int literal from parser"))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("all(")
                .ignore_then(expr.clone())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::All(Box::new(col), var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("exists(")
                .ignore_then(expr.clone())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::Exists(Box::new(col), var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("unique(")
                .ignore_then(expr.clone())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::Unique(Box::new(col), var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("count(")
                .ignore_then(expr.clone())
                .then_ignore(just(')'))
                .map(|col| ConstraintExpr::Count(Box::new(col)))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("templateVars(")
                .ignore_then(expr.clone())
                .then_ignore(just(')'))
                .map(|e| ConstraintExpr::TemplateVars(Box::new(e)))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("keys(")
                .ignore_then(expr.clone())
                .then_ignore(just(')'))
                .map(|e| ConstraintExpr::Keys(Box::new(e)))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("isPresent(")
                .ignore_then(ident())
                .then_ignore(just(')'))
                .map(|name| ConstraintExpr::IsPresent(name.node))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("isType(")
                .ignore_then(expr.clone())
                .then_ignore(ws().then(just(',')).then(ws()))
                .then(ident())
                .then_ignore(just(')'))
                .map(|(e, type_name)| ConstraintExpr::IsType(Box::new(e), type_name.node))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just('(')
                .ignore_then(ws_nl())
                .ignore_then(expr.clone())
                .then_ignore(ws_nl())
                .then_ignore(just(')')),
            ident()
                .map(|n| ConstraintExpr::Var(n.node))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
        ));

        let postfix = atom.foldl(just('.').ignore_then(ident()).repeated(), |left, field| {
            let span = left.span.start..field.span.end;
            Spanned::new(
                ConstraintExpr::FieldAccess(Box::new(left), field.node),
                span,
            )
        });

        let unary = choice((
            just('!')
                .ignore_then(postfix.clone())
                .map(|e| ConstraintExpr::Not(Box::new(e)))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            postfix.clone(),
        ));

        let cmp = unary.clone().foldl(
            ws().ignore_then(choice((
                just("==").to("=="),
                just("!=").to("!="),
                just("<=").to("<="),
                just(">=").to(">="),
                just('<').to("<"),
                just('>').to(">"),
                just("in").to("in"),
            )))
            .then_ignore(ws())
            .then(unary.clone())
            .repeated(),
            |left, (op, right)| {
                let span = left.span.start..right.span.end;
                let node = match op {
                    "==" => ConstraintExpr::Eq(Box::new(left), Box::new(right)),
                    "!=" => ConstraintExpr::Ne(Box::new(left), Box::new(right)),
                    "<=" => ConstraintExpr::Le(Box::new(left), Box::new(right)),
                    ">=" => ConstraintExpr::Ge(Box::new(left), Box::new(right)),
                    "<" => ConstraintExpr::Lt(Box::new(left), Box::new(right)),
                    ">" => ConstraintExpr::Gt(Box::new(left), Box::new(right)),
                    "in" => ConstraintExpr::In(Box::new(left), Box::new(right)),
                    _ => unreachable!(),
                };
                Spanned::new(node, span)
            },
        );

        let and = cmp.clone().foldl(
            ws().ignore_then(just("&&"))
                .ignore_then(ws())
                .ignore_then(cmp.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                Spanned::new(ConstraintExpr::And(Box::new(left), Box::new(right)), span)
            },
        );

        and.clone().foldl(
            ws().ignore_then(just("||"))
                .ignore_then(ws())
                .ignore_then(and.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                Spanned::new(ConstraintExpr::Or(Box::new(left), Box::new(right)), span)
            },
        )
    })
}

pub(super) fn annotation<'a>(
) -> impl Parser<'a, ParserInput<'a>, S<Annotation>, ParserExtra<'a>> + Clone {
    choice((
        just("@main")
            .to(Annotation::Main)
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
        just("@source")
            .ignore_then(ws())
            .ignore_then(just('['))
            .ignore_then(ws_nl())
            .ignore_then(
                source_path()
                    .separated_by(ws_nl().then(just(',')).then(ws_nl()))
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(ws_nl())
            .then_ignore(just(']'))
            .map(Annotation::Source)
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
        just("@out")
            .to(Annotation::Out)
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
        just("@constraint")
            .ignore_then(ws())
            .ignore_then(constraint_expr())
            .map(Annotation::Constraint)
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
        just("@doc")
            .ignore_then(ws())
            .ignore_then(just('"'))
            .ignore_then(none_of('"').repeated().to_slice())
            .then_ignore(just('"'))
            .map(|s: &str| Annotation::Doc(s.to_string()))
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
    ))
}

fn type_field<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<Field>, ParserExtra<'a>> + Clone {
    annotation()
        .then_ignore(ws_nl())
        .repeated()
        .collect::<Vec<_>>()
        .then(ident())
        .then(just('!').or_not().map(|o| o.is_none()))
        .then_ignore(ws())
        .then(type_expr)
        .map(|(((annotations, name), optional), ty)| Field {
            name,
            optional,
            ty,
            annotations,
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn struct_type<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    let anon_field = just('_').ignore_then(ws().ignore_then(type_expr.clone()).or_not());

    let anon_struct = just('{')
        .ignore_then(ws_nl())
        .ignore_then(
            anon_field
                .separated_by(sep())
                .at_least(1)
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(|fields: Vec<_>| TypeExpr::Struct(StructKind::Anonymous(fields)))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    let open_struct = just("{...}")
        .to(TypeExpr::Struct(StructKind::Open(vec![])))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    let empty_struct = just("{}")
        .to(TypeExpr::Struct(StructKind::Closed(vec![])))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    let named_struct = just('{')
        .ignore_then(ws_nl())
        .ignore_then(
            type_field(type_expr)
                .separated_by(sep())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .then_ignore(just('}'))
        .map(|fields| TypeExpr::Struct(StructKind::Closed(fields)))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    choice((open_struct, empty_struct, anon_struct, named_struct))
}

pub fn type_expr<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    recursive(|type_expr| {
        let atom = choice((
            lit_bool_type(),
            lit_string_type(),
            refinable_ref(), // -TypeName (before lit_int to avoid conflict with negative numbers)
            lit_int_type(),
            concrete(type_expr.clone()),
            base_type(),
            reference(),
            list_type(type_expr.clone()),
            struct_type(type_expr.clone()),
            named_type(),
        ));

        let intersection = atom.clone().foldl(
            ws().ignore_then(just('&'))
                .ignore_then(ws())
                .ignore_then(atom.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                Spanned::new(
                    TypeExpr::Intersection(Box::new(left), Box::new(right)),
                    span,
                )
            },
        );

        intersection.clone().foldl(
            ws().ignore_then(just('|'))
                .ignore_then(ws())
                .ignore_then(intersection.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                let mut variants = match left.node {
                    TypeExpr::Union(v) => v,
                    _ => vec![left],
                };
                match right.node {
                    TypeExpr::Union(v) => variants.extend(v),
                    _ => variants.push(right),
                }
                Spanned::new(TypeExpr::Union(variants), span)
            },
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_type(s: &str) -> S<TypeExpr> {
        type_expr().parse(s).into_result().unwrap()
    }

    fn parse_type_err(s: &str) {
        assert!(type_expr().parse(s).into_result().is_err());
    }

    fn parse_ann(s: &str) -> S<Annotation> {
        annotation().parse(s).into_result().unwrap()
    }

    // === Base types ===

    #[test]
    fn test_wildcard() {
        assert!(matches!(parse_type("*").node, TypeExpr::Base(BaseType::Wildcard)));
    }

    #[test]
    fn test_all_base_types() {
        let cases = [
            ("String", BaseType::String),
            ("Int", BaseType::Int),
            ("Float", BaseType::Float),
            ("Bool", BaseType::Bool),
            ("Uuid", BaseType::Uuid),
            ("Date", BaseType::Date),
            ("Timestamp", BaseType::Timestamp),
            ("Money", BaseType::Money),
        ];
        for (input, expected) in cases {
            assert_eq!(parse_type(input).node, TypeExpr::Base(expected));
        }
    }

    // === Literal types ===

    #[test]
    fn test_lit_string() {
        assert_eq!(parse_type("\"hello\"").node, TypeExpr::LitString("hello".into()));
    }

    #[test]
    fn test_lit_string_empty() {
        assert_eq!(parse_type("\"\"").node, TypeExpr::LitString("".into()));
    }

    #[test]
    fn test_lit_int_positive() {
        assert_eq!(parse_type("42").node, TypeExpr::LitInt(42));
    }

    #[test]
    fn test_lit_int_negative() {
        assert_eq!(parse_type("-7").node, TypeExpr::LitInt(-7));
    }

    #[test]
    fn test_lit_int_zero() {
        assert_eq!(parse_type("0").node, TypeExpr::LitInt(0));
    }

    #[test]
    fn test_lit_bool_true() {
        assert_eq!(parse_type("true").node, TypeExpr::LitBool(true));
    }

    #[test]
    fn test_lit_bool_false() {
        assert_eq!(parse_type("false").node, TypeExpr::LitBool(false));
    }

    // === Named types ===

    #[test]
    fn test_named_type() {
        assert_eq!(parse_type("Foo").node, TypeExpr::Named("Foo".into()));
    }

    #[test]
    fn test_named_type_lowercase() {
        assert_eq!(parse_type("myType").node, TypeExpr::Named("myType".into()));
    }

    #[test]
    fn test_named_rejects_reserved_words() {
        // Reserved words should parse as their specific type, not Named
        assert!(matches!(parse_type("String").node, TypeExpr::Base(_)));
        assert!(matches!(parse_type("true").node, TypeExpr::LitBool(_)));
        assert!(matches!(parse_type("false").node, TypeExpr::LitBool(_)));
    }

    // === Reference ===

    #[test]
    fn test_reference() {
        assert_eq!(parse_type("&Foo").node, TypeExpr::Reference("Foo".into()));
    }

    // === Refinable ref ===

    #[test]
    fn test_refinable_ref() {
        assert_eq!(parse_type("-Event").node, TypeExpr::RefinableRef("Event".into()));
    }

    #[test]
    fn test_refinable_ref_in_list() {
        let TypeExpr::List(card, elem) = parse_type("[]-Event").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::Any);
        assert_eq!(elem.node, TypeExpr::RefinableRef("Event".into()));
    }

    // === Concrete ===

    #[test]
    fn test_concrete_base() {
        let TypeExpr::Concrete(inner) = parse_type("Concrete<String>").node else {
            panic!("Expected Concrete");
        };
        assert_eq!(inner.node, TypeExpr::Base(BaseType::String));
    }

    #[test]
    fn test_concrete_named() {
        let TypeExpr::Concrete(inner) = parse_type("Concrete<Foo>").node else {
            panic!("Expected Concrete");
        };
        assert_eq!(inner.node, TypeExpr::Named("Foo".into()));
    }

    // === List types ===

    #[test]
    fn test_list_any() {
        let TypeExpr::List(card, inner) = parse_type("[]String").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::Any);
        assert_eq!(inner.node, TypeExpr::Base(BaseType::String));
    }

    #[test]
    fn test_list_exact() {
        let TypeExpr::List(card, _) = parse_type("[3]Int").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::Exact(3));
    }

    #[test]
    fn test_list_at_least() {
        let TypeExpr::List(card, _) = parse_type("[1..]Int").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::AtLeast(1));
    }

    #[test]
    fn test_list_at_most() {
        let TypeExpr::List(card, _) = parse_type("[..5]Int").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::AtMost(5));
    }

    #[test]
    fn test_list_range() {
        let TypeExpr::List(card, _) = parse_type("[2..10]Int").node else {
            panic!("Expected list");
        };
        assert_eq!(card, Cardinality::Range(2, 10));
    }

    #[test]
    fn test_list_of_structs() {
        let TypeExpr::List(_, inner) = parse_type("[]{x Int}").node else {
            panic!("Expected list");
        };
        assert!(matches!(inner.node, TypeExpr::Struct(StructKind::Closed(_))));
    }

    #[test]
    fn test_list_of_named() {
        let TypeExpr::List(_, inner) = parse_type("[]Foo").node else {
            panic!("Expected list");
        };
        assert_eq!(inner.node, TypeExpr::Named("Foo".into()));
    }

    // === Struct types ===

    #[test]
    fn test_empty_struct() {
        assert_eq!(
            parse_type("{}").node,
            TypeExpr::Struct(StructKind::Closed(vec![]))
        );
    }

    #[test]
    fn test_open_struct() {
        assert_eq!(
            parse_type("{...}").node,
            TypeExpr::Struct(StructKind::Open(vec![]))
        );
    }

    #[test]
    fn test_single_field_struct() {
        let TypeExpr::Struct(StructKind::Closed(fields)) = parse_type("{x Int}").node else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].node.name.node, "x");
        assert_eq!(fields[0].node.ty.node, TypeExpr::Base(BaseType::Int));
        assert!(fields[0].node.optional); // no `!` means optional=true (required marker is `!`)
    }

    #[test]
    fn test_required_field() {
        let TypeExpr::Struct(StructKind::Closed(fields)) = parse_type("{x! Int}").node else {
            panic!("Expected closed struct");
        };
        assert!(!fields[0].node.optional); // `!` means optional=false
    }

    #[test]
    fn test_multi_field_struct() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{x Int, y String}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].node.name.node, "x");
        assert_eq!(fields[1].node.name.node, "y");
    }

    #[test]
    fn test_struct_newline_separated() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{\n  x Int\n  y String\n}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_struct_trailing_comma() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{x Int, y String,}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_nested_struct() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{inner {x Int}}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields.len(), 1);
        assert!(matches!(
            fields[0].node.ty.node,
            TypeExpr::Struct(StructKind::Closed(_))
        ));
    }

    #[test]
    fn test_anonymous_struct_single() {
        let TypeExpr::Struct(StructKind::Anonymous(fields)) = parse_type("{_ String}").node else {
            panic!("Expected anonymous struct");
        };
        assert_eq!(fields.len(), 1);
        assert!(fields[0].is_some());
    }

    #[test]
    fn test_anonymous_struct_untyped() {
        let TypeExpr::Struct(StructKind::Anonymous(fields)) = parse_type("{_}").node else {
            panic!("Expected anonymous struct");
        };
        assert_eq!(fields.len(), 1);
        assert!(fields[0].is_none());
    }

    #[test]
    fn test_anonymous_struct_multi() {
        let TypeExpr::Struct(StructKind::Anonymous(fields)) =
            parse_type("{_ String, _ Int}").node
        else {
            panic!("Expected anonymous struct");
        };
        assert_eq!(fields.len(), 2);
    }

    #[test]
    fn test_field_with_annotation() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{@out\nx Int}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields[0].node.annotations.len(), 1);
        assert_eq!(fields[0].node.annotations[0].node, Annotation::Out);
    }

    #[test]
    fn test_field_with_source_annotation() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{@source [foo]\nx Int}").node
        else {
            panic!("Expected closed struct");
        };
        let Annotation::Source(paths) = &fields[0].node.annotations[0].node else {
            panic!("Expected Source annotation");
        };
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].node, SourcePath::Simple("foo".into()));
    }

    #[test]
    fn test_field_with_multiple_annotations() {
        let TypeExpr::Struct(StructKind::Closed(fields)) =
            parse_type("{@out\n@doc \"help\"\nx Int}").node
        else {
            panic!("Expected closed struct");
        };
        assert_eq!(fields[0].node.annotations.len(), 2);
    }

    // === Union ===

    #[test]
    fn test_union_two() {
        let TypeExpr::Union(variants) = parse_type("A | B").node else {
            panic!("Expected union");
        };
        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0].node, TypeExpr::Named("A".into()));
        assert_eq!(variants[1].node, TypeExpr::Named("B".into()));
    }

    #[test]
    fn test_union_three() {
        let TypeExpr::Union(variants) = parse_type("A | B | C").node else {
            panic!("Expected union");
        };
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_union_with_base_types() {
        let TypeExpr::Union(variants) = parse_type("String | Int").node else {
            panic!("Expected union");
        };
        assert_eq!(variants[0].node, TypeExpr::Base(BaseType::String));
        assert_eq!(variants[1].node, TypeExpr::Base(BaseType::Int));
    }

    #[test]
    fn test_union_with_literals() {
        let TypeExpr::Union(variants) = parse_type("\"a\" | \"b\" | \"c\"").node else {
            panic!("Expected union");
        };
        assert_eq!(variants.len(), 3);
        assert_eq!(variants[0].node, TypeExpr::LitString("a".into()));
    }

    // === Intersection ===

    #[test]
    fn test_intersection() {
        let TypeExpr::Intersection(left, right) = parse_type("{...} & {x Int}").node else {
            panic!("Expected intersection");
        };
        assert!(matches!(left.node, TypeExpr::Struct(StructKind::Open(_))));
        assert!(matches!(right.node, TypeExpr::Struct(StructKind::Closed(_))));
    }

    #[test]
    fn test_intersection_chain() {
        // a & b & c should be ((a & b) & c) due to left fold
        let TypeExpr::Intersection(left, right) = parse_type("A & B & C").node else {
            panic!("Expected intersection");
        };
        assert_eq!(right.node, TypeExpr::Named("C".into()));
        assert!(matches!(left.node, TypeExpr::Intersection(_, _)));
    }

    #[test]
    fn test_union_intersection_precedence() {
        // A | B & C should be A | (B & C) — intersection binds tighter
        let TypeExpr::Union(variants) = parse_type("A | B & C").node else {
            panic!("Expected union");
        };
        assert_eq!(variants.len(), 2);
        assert_eq!(variants[0].node, TypeExpr::Named("A".into()));
        assert!(matches!(variants[1].node, TypeExpr::Intersection(_, _)));
    }

    // === Annotations ===

    #[test]
    fn test_annotation_main() {
        assert_eq!(parse_ann("@main").node, Annotation::Main);
    }

    #[test]
    fn test_annotation_out() {
        assert_eq!(parse_ann("@out").node, Annotation::Out);
    }

    #[test]
    fn test_annotation_doc() {
        assert_eq!(parse_ann("@doc \"some help\"").node, Annotation::Doc("some help".into()));
    }

    #[test]
    fn test_annotation_source_simple() {
        let Annotation::Source(paths) = parse_ann("@source [foo]").node else {
            panic!("Expected Source");
        };
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].node, SourcePath::Simple("foo".into()));
    }

    #[test]
    fn test_annotation_source_dotted() {
        let Annotation::Source(paths) = parse_ann("@source [foo.bar.baz]").node else {
            panic!("Expected Source");
        };
        assert_eq!(paths[0].node, SourcePath::Dotted(vec!["foo".into(), "bar".into(), "baz".into()]));
    }

    #[test]
    fn test_annotation_source_multiple() {
        let Annotation::Source(paths) = parse_ann("@source [foo, bar]").node else {
            panic!("Expected Source");
        };
        assert_eq!(paths.len(), 2);
    }

    #[test]
    fn test_annotation_source_trailing_comma() {
        let Annotation::Source(paths) = parse_ann("@source [foo,]").node else {
            panic!("Expected Source");
        };
        assert_eq!(paths.len(), 1);
    }

    // === Constraint expressions ===

    #[test]
    fn test_constraint_bool() {
        let Annotation::Constraint(c) = parse_ann("@constraint true").node else {
            panic!("Expected Constraint");
        };
        assert_eq!(c.node, ConstraintExpr::Bool(true));
    }

    #[test]
    fn test_constraint_var() {
        let Annotation::Constraint(c) = parse_ann("@constraint x").node else {
            panic!("Expected Constraint");
        };
        assert_eq!(c.node, ConstraintExpr::Var("x".into()));
    }

    #[test]
    fn test_constraint_field_access() {
        let Annotation::Constraint(c) = parse_ann("@constraint x.y").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::FieldAccess(base, field) = c.node else {
            panic!("Expected FieldAccess");
        };
        assert_eq!(base.node, ConstraintExpr::Var("x".into()));
        assert_eq!(field, "y");
    }

    #[test]
    fn test_constraint_deep_field_access() {
        let Annotation::Constraint(c) = parse_ann("@constraint a.b.c").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::FieldAccess(inner, field) = c.node else {
            panic!("Expected FieldAccess");
        };
        assert_eq!(field, "c");
        assert!(matches!(inner.node, ConstraintExpr::FieldAccess(_, _)));
    }

    #[test]
    fn test_constraint_eq() {
        let Annotation::Constraint(c) = parse_ann("@constraint x == 1").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::Eq(_, _)));
    }

    #[test]
    fn test_constraint_ne() {
        let Annotation::Constraint(c) = parse_ann("@constraint x != y").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::Ne(_, _)));
    }

    #[test]
    fn test_constraint_lt_le_gt_ge() {
        assert!(matches!(parse_ann("@constraint x < 1").node, Annotation::Constraint(c) if matches!(c.node, ConstraintExpr::Lt(_, _))));
        assert!(matches!(parse_ann("@constraint x <= 1").node, Annotation::Constraint(c) if matches!(c.node, ConstraintExpr::Le(_, _))));
        assert!(matches!(parse_ann("@constraint x > 1").node, Annotation::Constraint(c) if matches!(c.node, ConstraintExpr::Gt(_, _))));
        assert!(matches!(parse_ann("@constraint x >= 1").node, Annotation::Constraint(c) if matches!(c.node, ConstraintExpr::Ge(_, _))));
    }

    #[test]
    fn test_constraint_in() {
        let Annotation::Constraint(c) = parse_ann("@constraint x in y").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::In(_, _)));
    }

    #[test]
    fn test_constraint_not() {
        let Annotation::Constraint(c) = parse_ann("@constraint !x").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::Not(inner) = c.node else { panic!("Expected Not"); };
        assert_eq!(inner.node, ConstraintExpr::Var("x".into()));
    }

    #[test]
    fn test_constraint_and() {
        let Annotation::Constraint(c) = parse_ann("@constraint a && b").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::And(_, _)));
    }

    #[test]
    fn test_constraint_or() {
        let Annotation::Constraint(c) = parse_ann("@constraint a || b").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::Or(_, _)));
    }

    #[test]
    fn test_constraint_and_or_precedence() {
        // a || b && c should be a || (b && c)
        let Annotation::Constraint(c) = parse_ann("@constraint a || b && c").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::Or(left, right) = c.node else { panic!("Expected Or"); };
        assert_eq!(left.node, ConstraintExpr::Var("a".into()));
        assert!(matches!(right.node, ConstraintExpr::And(_, _)));
    }

    #[test]
    fn test_constraint_parens() {
        let Annotation::Constraint(c) = parse_ann("@constraint (a || b) && c").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::And(left, right) = c.node else { panic!("Expected And"); };
        assert!(matches!(left.node, ConstraintExpr::Or(_, _)));
        assert_eq!(right.node, ConstraintExpr::Var("c".into()));
    }

    #[test]
    fn test_constraint_int() {
        let Annotation::Constraint(c) = parse_ann("@constraint x == -5").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::Eq(_, right) = c.node else { panic!("Expected Eq"); };
        assert_eq!(right.node, ConstraintExpr::Int(-5));
    }

    #[test]
    fn test_constraint_all() {
        let Annotation::Constraint(c) = parse_ann("@constraint all(items, i => i.valid)").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::All(col, var, body) = c.node else { panic!("Expected All"); };
        assert_eq!(col.node, ConstraintExpr::Var("items".into()));
        assert_eq!(var, "i");
        assert!(matches!(body.node, ConstraintExpr::FieldAccess(_, _)));
    }

    #[test]
    fn test_constraint_exists() {
        let Annotation::Constraint(c) = parse_ann("@constraint exists(xs, x => x == 1)").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::Exists(_, _, _)));
    }

    #[test]
    fn test_constraint_unique() {
        let Annotation::Constraint(c) = parse_ann("@constraint unique(xs, x => x.id)").node else {
            panic!("Expected Constraint");
        };
        assert!(matches!(c.node, ConstraintExpr::Unique(_, _, _)));
    }

    #[test]
    fn test_constraint_count() {
        let Annotation::Constraint(c) = parse_ann("@constraint count(items) > 0").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::Gt(left, _) = c.node else { panic!("Expected Gt"); };
        assert!(matches!(left.node, ConstraintExpr::Count(_)));
    }

    #[test]
    fn test_constraint_template_vars() {
        let Annotation::Constraint(c) = parse_ann("@constraint templateVars(x) in keys(y)").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::In(left, right) = c.node else { panic!("Expected In"); };
        assert!(matches!(left.node, ConstraintExpr::TemplateVars(_)));
        assert!(matches!(right.node, ConstraintExpr::Keys(_)));
    }

    #[test]
    fn test_constraint_is_present() {
        let Annotation::Constraint(c) = parse_ann("@constraint isPresent(foo)").node else {
            panic!("Expected Constraint");
        };
        assert_eq!(c.node, ConstraintExpr::IsPresent("foo".into()));
    }

    #[test]
    fn test_constraint_is_type() {
        let Annotation::Constraint(c) = parse_ann("@constraint isType(x, Foo)").node else {
            panic!("Expected Constraint");
        };
        let ConstraintExpr::IsType(expr, name) = c.node else { panic!("Expected IsType"); };
        assert_eq!(expr.node, ConstraintExpr::Var("x".into()));
        assert_eq!(name, "Foo");
    }

    // === Spans ===

    #[test]
    fn test_span_base_type() {
        let t = parse_type("String");
        assert_eq!(t.span, (0..6));
    }

    #[test]
    fn test_span_named() {
        let t = parse_type("Foo");
        assert_eq!(t.span, (0..3));
    }

    // === Error cases ===

    #[test]
    fn test_reject_empty() {
        parse_type_err("");
    }

    #[test]
    fn test_reject_unclosed_struct() {
        parse_type_err("{x Int");
    }
}
