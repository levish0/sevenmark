use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::{Location, ParserInput, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn token_underscore_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start() + parser_input.state.base_offset;
    literal("_").parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end() + parser_input.state.base_offset;

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: "_".to_string(),
    }))
}
