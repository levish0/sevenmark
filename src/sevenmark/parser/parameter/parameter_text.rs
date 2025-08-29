use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{Location, SevenMarkElement, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;

//
pub fn parameter_text_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start() + parser_input.state.base_offset;
    let content = take_while(1.., |c: char| c != '"' && c != '\\').parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end() + parser_input.state.base_offset;

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: content.to_string(),
    }))
}
