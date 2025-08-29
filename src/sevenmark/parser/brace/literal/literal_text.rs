use crate::sevenmark::ast::{Location, SevenMarkElement, TextElement};
use crate::sevenmark::ParserInput;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;
use winnow::Result;

/// Parse literal text within braces (excludes literal syntax symbols)
/// Reads characters except: {, }, \, newline
pub fn literal_text_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();
    let parsed_content = take_while(1.., |c: char| !matches!(c, '{' | '}' | '\\' | '\n'))
        .parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: parsed_content.to_string(),
    }))
}
