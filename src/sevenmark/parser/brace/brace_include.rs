use crate::sevenmark::ast::{Location, SevenMarkElement};
use crate::sevenmark::parser::brace::include::include_content_parser;
use crate::sevenmark::parser::parameter::parameter_core_parser;
use crate::sevenmark::{IncludeElement, ParserInput};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::{delimited, opt};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_include_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let ((parameters, _), parsed_content) = delimited(
        literal("{{{#include"),
        (
            (opt(parameter_core_parser), multispace0),
            |input: &mut ParserInput| {
                let mut inner_input = input.clone();
                inner_input
                    .state
                    .increase_depth()
                    .map_err(|e| e.into_context_error())?;
                let result = include_content_parser(&mut inner_input);
                inner_input
                    .state
                    .decrease_depth()
                    .map_err(|e| e.into_context_error())?;
                *input = inner_input;
                result
            },
        ),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Include(IncludeElement {
        location: Location { start, end },
        parameters: parameters.unwrap_or_default(),
        content: parsed_content,
    }))
}
