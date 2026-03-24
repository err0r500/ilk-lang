use chumsky::prelude::*;

use crate::span::{Spanned, S};

pub(super) type ParserInput<'a> = &'a str;
pub(super) type ParserExtra<'a> = extra::Err<Rich<'a, char>>;

pub(super) fn ident<'a>() -> impl Parser<'a, ParserInput<'a>, S<String>, ParserExtra<'a>> + Clone {
    text::ident().map_with(|s: &str, e| Spanned::from_simple(s.to_string(), e.span()))
}

pub(super) fn ws<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    one_of(" \t").repeated().ignored()
}

pub(super) fn ws_nl<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
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

pub(super) fn sep<'a>() -> impl Parser<'a, ParserInput<'a>, (), ParserExtra<'a>> + Clone {
    ws_nl().then(just(',').or_not()).then(ws_nl()).ignored()
}
