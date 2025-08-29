use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::{Location, ParserInput, TextElement};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use winnow::Result;

pub fn token_comma_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    if parser_input.state.inside_subscript {
        if parser_input.input.starts_with(",,") {
            return Err(winnow::error::ContextError::new());
        }
    }

    let start = parser_input.input.current_token_start();
    literal(",").parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: ",".to_string(),
    }))
}
