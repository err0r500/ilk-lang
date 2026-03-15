use crate::ast::*;
use crate::error::Diagnostic;
use crate::span::{Spanned, S};
use chumsky::prelude::*;
use std::path::Path;

type ParserInput<'a> = &'a str;
type ParserExtra<'a> = extra::Err<Rich<'a, char>>;

// ============= Common Parsers =============

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

// ============= Type Expression Parsers =============

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

fn lit_string_type<'a>() -> impl Parser<'a, ParserInput<'a>, S<TypeExpr>, ParserExtra<'a>> + Clone {
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
        just("..")
            .ignore_then(num.clone())
            .map(Cardinality::AtMost),
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
                "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
                    | "true" | "false" | "Concrete" | "forall" | "exists" | "unique" | "count"
                    | "templateVars" | "keys" | "in" | "type" | "import"
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

fn constraint_expr<'a>() -> impl Parser<'a, ParserInput<'a>, S<ConstraintExpr>, ParserExtra<'a>> + Clone {
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
                .map(|s: &str| ConstraintExpr::Int(s.parse().expect("valid int literal from parser")))
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
            refinable_ref(),  // -TypeName (before lit_int to avoid conflict with negative numbers)
            lit_int_type(),
            concrete(type_expr.clone()),
            base_type(),
            reference(),
            list_type(type_expr.clone()),
            struct_type(type_expr.clone()),
            named_type(),
        ));

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

        intersection.clone().foldl(
            ws()
                .ignore_then(just('|'))
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

// ============= Value Parsers (Instance-level) =============

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
        just(" = ")
            .ignore_then(dot_path())
            .map(FieldOrigin::Mapped),
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

fn refinement_field<'a>() -> impl Parser<'a, ParserInput<'a>, S<InstanceField>, ParserExtra<'a>> + Clone {
    let simple_value = choice((type_ref_value(), lit_int_value(), lit_string_value(), lit_bool_value()));

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
        .map(|((((doc, name), optional), value), origin)| InstanceField {
            name,
            optional,
            value,
            origin,
            doc: doc.map(|s: &str| s.to_string()),
        })
        .map_with(|f, e| Spanned::from_simple(f, e.span()))
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
    text::ident()
        .filter(|s: &&str| {
            !matches!(
                *s,
                "Uuid" | "String" | "Int" | "Float" | "Bool" | "Date" | "Timestamp" | "Money"
                    | "true" | "false" | "type" | "import"
            )
        })
        .map(|s: &str| Value::BindingRef(s.to_string()))
        .map_with(|v, e| Spanned::from_simple(v, e.span()))
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

// ============= Top-Level Item Parsers =============

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

// type Name = TypeExpr
fn type_decl<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    annotation()
        .then_ignore(ws_nl())
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(just("type"))
        .then_ignore(ws())
        .then(ident())
        .then_ignore(ws())
        .then_ignore(just('='))
        .then_ignore(ws())
        .then(type_expr())
        .map(|((annotations, name), body)| {
            Item::TypeDecl(TypeDecl {
                name,
                annotations,
                body,
            })
        })
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

// name = TypeName<assocs> body
fn instance<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    let doc = just("@doc")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .then_ignore(ws_nl())
        .or_not();

    // Collect @main and @doc annotations
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
        .then(assocs())
        .then_ignore(ws())
        .then(value())
        .map(|(((((is_main, doc), name), type_name), assocs), body)| {
            let annotations = if is_main {
                vec![Spanned::new(Annotation::Main, 0..0)]
            } else {
                vec![]
            };
            Item::Instance(Instance {
                name,
                type_name,
                assocs,
                body,
                annotations,
                doc: doc.map(|s: &str| s.to_string()),
            })
        })
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

// import "path" [as alias]
fn import<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    just("import")
        .ignore_then(ws())
        .ignore_then(just('"'))
        .ignore_then(none_of('"').repeated().to_slice())
        .then_ignore(just('"'))
        .map_with(|s: &str, e| Spanned::from_simple(s.to_string(), e.span()))
        .then(
            ws()
                .ignore_then(just("as"))
                .ignore_then(ws())
                .ignore_then(ident())
                .or_not(),
        )
        .map(|(path, alias)| {
            Item::Import(Import { path, alias })
        })
        .map_with(|i, e| Spanned::from_simple(i, e.span()))
}

fn item<'a>() -> impl Parser<'a, ParserInput<'a>, S<Item>, ParserExtra<'a>> + Clone {
    choice((import(), type_decl(), instance()))
}

pub fn file<'a>() -> impl Parser<'a, ParserInput<'a>, File, ParserExtra<'a>> {
    ws_nl()
        .ignore_then(
            item()
                .separated_by(ws_nl())
                .allow_trailing()
                .collect::<Vec<_>>(),
        )
        .then_ignore(ws_nl())
        .map(|items| File { items })
}

pub fn parse(src: &str, path: &Path) -> Result<File, Vec<Diagnostic>> {
    file()
        .parse(src)
        .into_result()
        .map_err(|errs| {
            errs.into_iter()
                .map(|e| {
                    Diagnostic::error(
                        e.span().into_range(),
                        e.to_string(),
                        path.to_path_buf(),
                    )
                })
                .collect()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_type(s: &str) -> S<TypeExpr> {
        type_expr().parse(s).into_result().unwrap()
    }

    fn parse_value(s: &str) -> S<Value> {
        value().parse(s).into_result().unwrap()
    }

    fn parse_file(s: &str) -> File {
        file().parse(s).into_result().unwrap()
    }

    // Type parsing tests
    #[test]
    fn test_wildcard() {
        assert!(matches!(parse_type("*").node, TypeExpr::Base(BaseType::Wildcard)));
    }

    #[test]
    fn test_base_types() {
        assert!(matches!(parse_type("String").node, TypeExpr::Base(BaseType::String)));
        assert!(matches!(parse_type("Int").node, TypeExpr::Base(BaseType::Int)));
    }

    #[test]
    fn test_concrete() {
        let t = parse_type("Concrete<String>");
        assert!(matches!(t.node, TypeExpr::Concrete(_)));
    }

    #[test]
    fn test_union() {
        let t = parse_type("A | B");
        if let TypeExpr::Union(variants) = t.node {
            assert_eq!(variants.len(), 2);
        } else {
            panic!("Expected union");
        }
    }

    #[test]
    fn test_refinable_ref() {
        let t = parse_type("-Event");
        assert!(matches!(t.node, TypeExpr::RefinableRef(name) if name == "Event"));
    }

    #[test]
    fn test_refinable_ref_in_list() {
        let t = parse_type("[]-Event");
        if let TypeExpr::List(_, elem) = t.node {
            assert!(matches!(elem.node, TypeExpr::RefinableRef(name) if name == "Event"));
        } else {
            panic!("Expected list");
        }
    }

    // Value parsing tests
    #[test]
    fn test_value_type_ref() {
        assert!(matches!(parse_value("String").node, Value::TypeRef(s) if s == "String"));
    }

    #[test]
    fn test_value_struct() {
        let v = parse_value("{x String}");
        if let Value::Struct(fields) = v.node {
            assert_eq!(fields.len(), 1);
        } else {
            panic!("Expected struct");
        }
    }

    // File parsing tests
    #[test]
    fn test_type_decl() {
        let f = parse_file("type Foo = {x Int}");
        assert_eq!(f.type_decls().count(), 1);
    }

    #[test]
    fn test_instance() {
        let f = parse_file("foo = Foo {x Int}");
        assert_eq!(f.instances().count(), 1);
    }

    #[test]
    fn test_instance_with_assocs() {
        let f = parse_file("foo = Foo<a, b> {x Int}");
        let inst = f.instances().next().unwrap();
        assert_eq!(inst.assocs.len(), 2);
    }

    #[test]
    fn test_import() {
        let f = parse_file("import \"./base.ilk\"");
        assert_eq!(f.imports().count(), 1);
    }

    #[test]
    fn test_import_with_alias() {
        let f = parse_file("import \"./base.ilk\" as base");
        let imp = f.imports().next().unwrap();
        assert!(imp.alias.is_some());
    }

    #[test]
    fn test_main_instance() {
        let f = parse_file("@main\nboard = Board {x Int}");
        let inst = f.instances().next().unwrap();
        assert!(inst.annotations.iter().any(|a| matches!(a.node, Annotation::Main)));
    }

    #[test]
    fn test_full_file() {
        let src = r#"
type Tag = {_ String}

@assoc [Tag]
type Event = {...} & {timestamp Int}

tag1 = Tag {x String}

ev = Event<tag1> {
    id String
}

@main
board = Board {
    events [ev]
}
"#;
        let f = parse_file(src);
        assert_eq!(f.type_decls().count(), 2);
        assert_eq!(f.instances().count(), 3);
    }
}
