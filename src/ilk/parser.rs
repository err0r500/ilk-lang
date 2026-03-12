use crate::error::Diagnostic;
use crate::ilk::ast::*;
use crate::span::{Spanned, S};
use chumsky::prelude::*;
use std::path::Path;

type ParserInput<'a> = &'a str;
type ParserExtra<'a> = extra::Err<Rich<'a, char>>;

fn ident<'a>() -> impl Parser<'a, ParserInput<'a>, S<String>, ParserExtra<'a>> + Clone {
    text::ident()
        .map_with(|s: &str, e| Spanned::from_simple(s.to_string(), e.span()))
}

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

fn lit_string<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map(|s: &str| TypeExpr::LitString(s.to_string()))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn lit_int<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    just('-')
        .or_not()
        .then(text::int(10))
        .to_slice()
        .map(|s: &str| TypeExpr::LitInt(s.parse().unwrap()))
        .map_with(|t, e| Spanned::from_simple(t, e.span()))
}

fn lit_bool<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
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

// Separator for struct fields and list elements: whitespace, then comma or newline, then whitespace
fn sep<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    ws_nl()
        .then(just(',').or_not())
        .then(ws_nl())
        .ignored()
}

fn cardinality<'a>() -> impl Parser<'a, ParserInput<'a>, Cardinality, ParserExtra<'a>> + Clone {
    let num = text::int(10).map(|s: &str| s.parse::<usize>().unwrap());

    choice((
        // [N..M]
        num.clone()
            .then_ignore(just(".."))
            .then(num.clone())
            .map(|(n, m)| Cardinality::Range(n, m)),
        // [N..]
        num.clone()
            .then_ignore(just(".."))
            .map(Cardinality::AtLeast),
        // [..M]
        just("..")
            .ignore_then(num.clone())
            .map(Cardinality::AtMost),
        // [N]
        num.map(Cardinality::Exact),
        // []
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
                "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
                    | "true" | "false" | "Concrete" | "forall" | "exists" | "unique" | "count"
                    | "templateVars" | "keys" | "in"
            )
        })
        .map(|s: &str| TypeExpr::Named(s.to_string()))
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

fn constraint_expr<'a>() -> impl Parser<'a, ParserInput<'a>, S<ConstraintExpr>, ParserExtra<'a>> + Clone
{
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
                .map(|s: &str| ConstraintExpr::Int(s.parse().unwrap()))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("forall(")
                .ignore_then(ident())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::ForAll(col.node, var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            // forall with expression collection (e.g., forall(templateVars(path), v => ...))
            just("forall(")
                .ignore_then(just("templateVars("))
                .ignore_then(expr.clone())
                .then_ignore(just(')'))
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col_expr, var), body)| {
                    let tv = ConstraintExpr::TemplateVars(Box::new(col_expr));
                    ConstraintExpr::ForAllExpr(Box::new(Spanned::new(tv, 0..0)), var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("exists(")
                .ignore_then(ident())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::Exists(col.node, var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("unique(")
                .ignore_then(ident())
                .then_ignore(just(',').then(ws()))
                .then(ident())
                .then_ignore(ws().then(just("=>")).then(ws()))
                .then(expr.clone())
                .then_ignore(just(')'))
                .map(|((col, var), body)| {
                    ConstraintExpr::Unique(col.node, var.node, Box::new(body))
                })
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            just("count(")
                .ignore_then(ident())
                .then_ignore(just(')'))
                .map(|col| ConstraintExpr::Count(col.node))
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
            just('(')
                .ignore_then(ws_nl())
                .ignore_then(expr.clone())
                .then_ignore(ws_nl())
                .then_ignore(just(')')),
            just('!')
                .ignore_then(expr.clone())
                .map(|e| ConstraintExpr::Not(Box::new(e)))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
            ident()
                .map(|n| ConstraintExpr::Var(n.node))
                .map_with(|c, e| Spanned::from_simple(c, e.span())),
        ));

        // Handle field access and method calls like e.assoc(t)
        let postfix = atom.foldl(
            choice((
                just(".assoc(")
                    .ignore_then(expr.clone())
                    .then_ignore(just(')'))
                    .map(|arg| (true, arg)),
                just('.')
                    .ignore_then(ident())
                    .map(|field| (false, Spanned::from_simple(ConstraintExpr::Var(field.node.clone()), field.span.into()))),
            ))
            .repeated(),
            |left, (is_assoc, right)| {
                let span = left.span.start..right.span.end;
                if is_assoc {
                    Spanned::new(
                        ConstraintExpr::Assoc(Box::new(left), Box::new(right)),
                        span,
                    )
                } else {
                    if let ConstraintExpr::Var(field) = &right.node {
                        Spanned::new(
                            ConstraintExpr::FieldAccess(Box::new(left), field.clone()),
                            span,
                        )
                    } else {
                        unreachable!()
                    }
                }
            },
        );

        // Comparison operators
        let cmp = postfix.clone().foldl(
            ws()
                .ignore_then(choice((
                    just("==").to("=="),
                    just("!=").to("!="),
                    just("<=").to("<="),
                    just(">=").to(">="),
                    just('<').to("<"),
                    just('>').to(">"),
                    just("in").to("in"),
                )))
                .then_ignore(ws())
                .then(postfix.clone())
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

        // Logical AND
        let and = cmp.clone().foldl(
            ws()
                .ignore_then(just("&&"))
                .ignore_then(ws())
                .ignore_then(cmp.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                Spanned::new(ConstraintExpr::And(Box::new(left), Box::new(right)), span)
            },
        );

        // Logical OR
        and.clone().foldl(
            ws()
                .ignore_then(just("||"))
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

fn annotation<'a>() -> impl Parser<'a, ParserInput<'a>, S<Annotation>, ParserExtra<'a>> + Clone {
    choice((
        just("@main")
            .to(Annotation::Main)
            .map_with(|a, e| Spanned::from_simple(a, e.span())),
        just("@assoc")
            .ignore_then(ws())
            .ignore_then(just('['))
            .ignore_then(ws_nl())
            .ignore_then(
                ident()
                    .separated_by(ws_nl().then(just(',')).then(ws_nl()))
                    .allow_trailing()
                    .collect::<Vec<_>>(),
            )
            .then_ignore(ws_nl())
            .then_ignore(just(']'))
            .map(Annotation::Assoc)
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

fn field<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, S<Field>, ParserExtra<'a>> + Clone {
    annotation()
        .then_ignore(ws_nl())
        .repeated()
        .collect::<Vec<_>>()
        .then(ident())
        .then_ignore(ws())
        .then(type_expr)
        .map(|((annotations, name), ty)| Field {
            name,
            ty,
            annotations,
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
}

fn struct_type<'a>(
    type_expr: impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone + 'a,
) -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
    // Anonymous struct: {_}, {_ Type}, {_, _}, etc.
    let anon_field = just('_')
        .ignore_then(ws().ignore_then(type_expr.clone()).or_not());

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

    // Open struct: {...}
    let open_struct = just("{...}")
        .to(TypeExpr::Struct(StructKind::Open(vec![])))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    // Empty struct: {}
    let empty_struct = just("{}")
        .to(TypeExpr::Struct(StructKind::Closed(vec![])))
        .map_with(|t, e| Spanned::from_simple(t, e.span()));

    // Named fields struct
    let named_struct = just('{')
        .ignore_then(ws_nl())
        .ignore_then(
            field(type_expr)
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
            lit_bool(),
            lit_string(),
            lit_int(),
            concrete(type_expr.clone()),
            base_type(),
            reference(),
            list_type(type_expr.clone()),
            struct_type(type_expr.clone()),
            named_type(),
        ));

        // Handle intersection (& binds tighter than |)
        let intersection = atom.clone().foldl(
            ws()
                .ignore_then(just('&'))
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

        // Handle union
        intersection.clone().foldl(
            ws()
                .ignore_then(just('|'))
                .ignore_then(ws())
                .ignore_then(intersection.clone())
                .repeated(),
            |left, right| {
                let span = left.span.start..right.span.end;
                // Flatten unions
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

fn block<'a>() -> impl Parser<'a, ParserInput<'a>, S<Block>, ParserExtra<'a>> + Clone {
    annotation()
        .then_ignore(ws_nl())
        .repeated()
        .collect::<Vec<_>>()
        .then(ident())
        .then_ignore(ws())
        .then(type_expr())
        .map(|((annotations, name), body)| Block {
            name,
            annotations,
            body,
        })
        .map_with(|b, e| Spanned::from_simple(b, e.span()))
}

pub fn ilk_file<'a>() -> impl Parser<'a, ParserInput<'a>, IlkFile, ParserExtra<'a>> {
    ws_nl()
        .ignore_then(
            block()
                .separated_by(ws_nl())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .map(|blocks| IlkFile { blocks })
}

pub fn parse_ilk(src: &str, file: &Path) -> Result<IlkFile, Vec<Diagnostic>> {
    ilk_file()
        .parse(src)
        .into_result()
        .map_err(|errs| {
            errs.into_iter()
                .map(|e| {
                    Diagnostic::error(
                        e.span().into_range(),
                        e.to_string(),
                        file.to_path_buf(),
                    )
                })
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(s: &str) -> S<TypeExpr> {
        type_expr().parse(s).into_result().unwrap()
    }

    fn parse_c(s: &str) -> S<ConstraintExpr> {
        constraint_expr().parse(s).into_result().unwrap()
    }

    // Phase 2.1: Base types & literals
    #[test]
    fn test_wildcard() {
        assert!(matches!(parse("*").node, TypeExpr::Base(BaseType::Wildcard)));
    }

    #[test]
    fn test_base_types() {
        assert!(matches!(parse("String").node, TypeExpr::Base(BaseType::String)));
        assert!(matches!(parse("Int").node, TypeExpr::Base(BaseType::Int)));
        assert!(matches!(parse("Uuid").node, TypeExpr::Base(BaseType::Uuid)));
        assert!(matches!(parse("Float").node, TypeExpr::Base(BaseType::Float)));
        assert!(matches!(parse("Bool").node, TypeExpr::Base(BaseType::Bool)));
        assert!(matches!(parse("Date").node, TypeExpr::Base(BaseType::Date)));
        assert!(matches!(parse("Timestamp").node, TypeExpr::Base(BaseType::Timestamp)));
        assert!(matches!(parse("Money").node, TypeExpr::Base(BaseType::Money)));
    }

    #[test]
    fn test_concrete() {
        let t = parse("Concrete<String>");
        assert!(matches!(t.node, TypeExpr::Concrete(_)));
    }

    #[test]
    fn test_literals() {
        assert!(matches!(parse("\"hello\"").node, TypeExpr::LitString(s) if s == "hello"));
        assert!(matches!(parse("42").node, TypeExpr::LitInt(42)));
        assert!(matches!(parse("-5").node, TypeExpr::LitInt(-5)));
        assert!(matches!(parse("true").node, TypeExpr::LitBool(true)));
        assert!(matches!(parse("false").node, TypeExpr::LitBool(false)));
    }

    // Phase 2.2: Structs
    #[test]
    fn test_empty_struct() {
        assert!(matches!(parse("{}").node, TypeExpr::Struct(StructKind::Closed(f)) if f.is_empty()));
    }

    #[test]
    fn test_open_struct() {
        assert!(matches!(parse("{...}").node, TypeExpr::Struct(StructKind::Open(_))));
    }

    #[test]
    fn test_anonymous_struct() {
        assert!(matches!(parse("{_}").node, TypeExpr::Struct(StructKind::Anonymous(v)) if v.len() == 1 && v[0].is_none()));
        assert!(matches!(parse("{_ String}").node, TypeExpr::Struct(StructKind::Anonymous(v)) if v.len() == 1 && v[0].is_some()));
        assert!(matches!(parse("{_, _}").node, TypeExpr::Struct(StructKind::Anonymous(v)) if v.len() == 2));
    }

    #[test]
    fn test_named_struct() {
        let t = parse("{x Int}");
        if let TypeExpr::Struct(StructKind::Closed(fields)) = t.node {
            assert_eq!(fields.len(), 1);
            assert_eq!(fields[0].node.name.node, "x");
        } else {
            panic!("Expected closed struct");
        }
    }

    #[test]
    fn test_multiline_struct() {
        let t = parse("{\n  x Int\n  y String\n}");
        if let TypeExpr::Struct(StructKind::Closed(fields)) = t.node {
            assert_eq!(fields.len(), 2);
        } else {
            panic!("Expected closed struct");
        }
    }

    // Phase 2.3: Lists
    #[test]
    fn test_list_any() {
        let t = parse("[]Event");
        assert!(matches!(t.node, TypeExpr::List(Cardinality::Any, _)));
    }

    #[test]
    fn test_list_exact() {
        let t = parse("[3]Tag");
        assert!(matches!(t.node, TypeExpr::List(Cardinality::Exact(3), _)));
    }

    #[test]
    fn test_list_at_least() {
        let t = parse("[1..]Tag");
        assert!(matches!(t.node, TypeExpr::List(Cardinality::AtLeast(1), _)));
    }

    #[test]
    fn test_list_at_most() {
        let t = parse("[..10]Tag");
        assert!(matches!(t.node, TypeExpr::List(Cardinality::AtMost(10), _)));
    }

    #[test]
    fn test_list_range() {
        let t = parse("[2..5]Tag");
        assert!(matches!(t.node, TypeExpr::List(Cardinality::Range(2, 5), _)));
    }

    // Phase 2.4: References
    #[test]
    fn test_reference() {
        assert!(matches!(parse("&Event").node, TypeExpr::Reference(n) if n == "Event"));
    }

    #[test]
    fn test_list_of_refs() {
        let t = parse("[]&Event");
        if let TypeExpr::List(Cardinality::Any, inner) = t.node {
            assert!(matches!(inner.node, TypeExpr::Reference(_)));
        } else {
            panic!("Expected list");
        }
    }

    // Phase 2.5: Union & Intersection
    #[test]
    fn test_union() {
        let t = parse("A | B");
        if let TypeExpr::Union(variants) = t.node {
            assert_eq!(variants.len(), 2);
        } else {
            panic!("Expected union");
        }
    }

    #[test]
    fn test_union_three() {
        let t = parse("A | B | C");
        if let TypeExpr::Union(variants) = t.node {
            assert_eq!(variants.len(), 3);
        } else {
            panic!("Expected union");
        }
    }

    #[test]
    fn test_intersection() {
        let t = parse("A & B");
        assert!(matches!(t.node, TypeExpr::Intersection(_, _)));
    }

    #[test]
    fn test_intersection_open_struct() {
        let t = parse("{...} & {x Int}");
        assert!(matches!(t.node, TypeExpr::Intersection(_, _)));
    }

    #[test]
    fn test_precedence_union_intersection() {
        // & binds tighter than |
        let t = parse("A | B & C");
        if let TypeExpr::Union(variants) = &t.node {
            assert_eq!(variants.len(), 2);
            assert!(matches!(&variants[0].node, TypeExpr::Named(n) if n == "A"));
            assert!(matches!(&variants[1].node, TypeExpr::Intersection(_, _)));
        } else {
            panic!("Expected union");
        }
    }

    // Phase 2.8: Constraints
    #[test]
    fn test_constraint_bool() {
        assert!(matches!(parse_c("true").node, ConstraintExpr::Bool(true)));
        assert!(matches!(parse_c("false").node, ConstraintExpr::Bool(false)));
    }

    #[test]
    fn test_constraint_var() {
        assert!(matches!(parse_c("x").node, ConstraintExpr::Var(n) if n == "x"));
    }

    #[test]
    fn test_constraint_field_access() {
        let c = parse_c("x.field");
        assert!(matches!(c.node, ConstraintExpr::FieldAccess(_, f) if f == "field"));
    }

    #[test]
    fn test_constraint_forall() {
        let c = parse_c("forall(col, x => true)");
        assert!(matches!(c.node, ConstraintExpr::ForAll(col, var, _) if col == "col" && var == "x"));
    }

    #[test]
    fn test_constraint_exists() {
        let c = parse_c("exists(tags, t => t.active)");
        assert!(matches!(c.node, ConstraintExpr::Exists(_, _, _)));
    }

    #[test]
    fn test_constraint_unique() {
        let c = parse_c("unique(items, i => i.id)");
        assert!(matches!(c.node, ConstraintExpr::Unique(_, _, _)));
    }

    #[test]
    fn test_constraint_count() {
        let c = parse_c("count(tags)");
        assert!(matches!(c.node, ConstraintExpr::Count(n) if n == "tags"));
    }

    #[test]
    fn test_constraint_assoc() {
        let c = parse_c("e.assoc(t)");
        assert!(matches!(c.node, ConstraintExpr::Assoc(_, _)));
    }

    #[test]
    fn test_constraint_template_vars() {
        let c = parse_c("templateVars(path)");
        assert!(matches!(c.node, ConstraintExpr::TemplateVars(_)));
    }

    #[test]
    fn test_constraint_keys() {
        let c = parse_c("keys(params)");
        assert!(matches!(c.node, ConstraintExpr::Keys(_)));
    }

    #[test]
    fn test_constraint_operators() {
        assert!(matches!(parse_c("a && b").node, ConstraintExpr::And(_, _)));
        assert!(matches!(parse_c("a || b").node, ConstraintExpr::Or(_, _)));
        assert!(matches!(parse_c("!a").node, ConstraintExpr::Not(_)));
        assert!(matches!(parse_c("a == b").node, ConstraintExpr::Eq(_, _)));
        assert!(matches!(parse_c("a != b").node, ConstraintExpr::Ne(_, _)));
        assert!(matches!(parse_c("x in set").node, ConstraintExpr::In(_, _)));
        assert!(matches!(parse_c("count(x) >= 1").node, ConstraintExpr::Ge(_, _)));
    }

    #[test]
    fn test_constraint_complex() {
        let c = parse_c("forall(tags, t => forall(events, e => e.assoc(t)))");
        assert!(matches!(c.node, ConstraintExpr::ForAll(_, _, _)));
    }

    #[test]
    fn test_constraint_template_vars_complex() {
        let c = parse_c("forall(templateVars(path), v => v in keys(params))");
        assert!(matches!(c.node, ConstraintExpr::ForAllExpr(_, _, _)));
    }

    // Phase 2.9: Full ilk file
    #[test]
    fn test_parse_dcb_board_spec() {
        let src = std::fs::read_to_string("examples/dcb-board-spec.ilk").unwrap();
        let result = parse_ilk(&src, Path::new("examples/dcb-board-spec.ilk"));
        assert!(result.is_ok(), "Parse error: {:?}", result.err());
        let file = result.unwrap();
        assert!(!file.blocks.is_empty());
    }
}
