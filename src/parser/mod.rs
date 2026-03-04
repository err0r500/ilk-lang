pub mod instance;
pub mod schema;

use chumsky::prelude::*;

pub type ParserInput<'a> = &'a str;
pub type ParserExtra<'a> = extra::Err<Rich<'a, char>>;

pub fn ident<'a>() -> impl Parser<'a, ParserInput<'a>, String, ParserExtra<'a>> + Clone {
    text::ident().map(|s: &str| s.to_string())
}

pub fn string_literal<'a>() -> impl Parser<'a, ParserInput<'a>, String, ParserExtra<'a>> + Clone {
    let escape = just('\\').ignore_then(
        just('\\')
            .or(just('/'))
            .or(just('"'))
            .or(just('n').to('\n'))
            .or(just('r').to('\r'))
            .or(just('t').to('\t')),
    );

    just('"')
        .ignore_then(none_of("\\\"").or(escape).repeated().collect::<String>())
        .then_ignore(just('"'))
}

pub fn int<'a>() -> impl Parser<'a, ParserInput<'a>, i64, ParserExtra<'a>> + Clone {
    let digits = text::digits(10).to_slice();
    just('-')
        .or_not()
        .then(digits)
        .to_slice()
        .map(|s: &str| s.parse().unwrap())
}

pub fn float<'a>() -> impl Parser<'a, ParserInput<'a>, f64, ParserExtra<'a>> + Clone {
    let digits = text::digits(10).to_slice();
    let frac = just('.').then(digits);
    let exp = just('e')
        .or(just('E'))
        .then(just('+').or(just('-')).or_not())
        .then(digits);

    just('-')
        .or_not()
        .then(digits)
        .then(frac)
        .then(exp.or_not())
        .to_slice()
        .map(|s: &str| s.parse().unwrap())
}

pub fn number<'a>() -> impl Parser<'a, ParserInput<'a>, NumericValue, ParserExtra<'a>> + Clone {
    float()
        .map(NumericValue::Float)
        .or(int().map(NumericValue::Int))
}

#[derive(Debug, Clone, PartialEq)]
pub enum NumericValue {
    Int(i64),
    Float(f64),
}

pub fn ws<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    let line_comment = just("//")
        .then(any().and_is(just('\n').not()).repeated())
        .ignored();
    let block_comment = just("/*").then(any().and_is(just("*/").not()).repeated()).then(just("*/")).ignored();

    choice((
        text::whitespace().at_least(1).ignored(),
        line_comment,
        block_comment,
    ))
    .repeated()
    .ignored()
}

pub fn padded<'a, T>(
    p: impl Parser<'a, ParserInput<'a>, T, ParserExtra<'a>> + Clone,
) -> impl Parser<'a, ParserInput<'a>, T, ParserExtra<'a>> + Clone {
    ws().ignore_then(p).then_ignore(ws())
}
