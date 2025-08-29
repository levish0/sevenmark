use crate::sevenmark::ast::{Location, SevenMarkElement, TextElement};
use crate::sevenmark::ParserInput;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;
use winnow::Result;

/// Parse plain text content within parameter values
/// Reads all characters except for quotes (") and backslashes (\)
/// which are handled by escape sequences or mark the end of parameter values
pub fn parameter_text_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();
    // Take all characters except quotes and backslashes
    let parsed_content =
        take_while(1.., |c: char| c != '"' && c != '\\').parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: parsed_content.to_string(),
    }))
}
