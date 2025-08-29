use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{Parameters, SevenMarkElement};
use crate::sevenmark::parser::parameter::parameter_content::parameter_content_parser;
use std::collections::HashMap;
use winnow::Result;
use winnow::ascii::{alphanumeric1, multispace0};
use winnow::combinator::{delimited, opt, preceded, repeat, terminated};
use winnow::prelude::*;
use winnow::token::literal;

fn parameter_parser(parser_input: &mut ParserInput) -> Result<(String, Vec<SevenMarkElement>)> {
    // #style="xx" @@@
    let (_, key, value_opt, _) = (
        multispace0,
        preceded(literal('#'), alphanumeric1),
        opt(preceded(
            literal('='),
            delimited(literal('"'), parameter_content_parser, literal('"')),
        )),
        multispace0,
    )
        .parse_next(parser_input)?;

    let key = key.to_string();
    let value = value_opt.unwrap_or_else(Vec::new);

    Ok((key, value))
}

pub fn parameter_core_parser(parser_input: &mut ParserInput) -> Result<Parameters> {
    terminated(
        repeat(1.., parameter_parser)
            .map(|pairs: Vec<_>| pairs.into_iter().collect::<HashMap<_, _>>()),
        preceded(opt(literal("||")), multispace0),
    )
    .parse_next(parser_input)
}
