use super::super::element::element_parser;
use crate::sevenmark::ast::{Location, SevenMarkElement, TextStyle};
use crate::sevenmark::{InputSource, ParserInput};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_until};

pub fn markdown_underline_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    let start = parser_input.input.current_token_start() + parser_input.state.base_offset;
    let parsed_content = delimited(
        literal("__"),
        take_until(0.., "__").verify(|s: &str| !s.contains('\n')),
        literal("__"),
    )
    .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end() + parser_input.state.base_offset;

    let new_state = parser_input
        .state
        .with_offset(start + 2)
        .map_err(|e| e.into_context_error())?;

    let mut content_stateful = ParserInput {
        input: InputSource::new(parsed_content),
        state: new_state,
    };

    let parsed_content = element_parser(&mut content_stateful)?;

    Ok(vec![SevenMarkElement::Underline(TextStyle {
        location: Location { start, end },
        content: parsed_content,
    })])
}
