use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::{Location, ParserInput, TextElement};
use winnow::Result;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn token_asterisk_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    // Bold context에서 **을 만나면 실패 (delimiter로 사용되어야 함)
    if parser_input.state.inside_bold {
        if parser_input.input.starts_with("**") {
            return Err(winnow::error::ContextError::new());
        }
    }
    // Italic context에서 *을 만나면 실패 (delimiter로 사용되어야 함)
    if parser_input.state.inside_italic {
        if parser_input.input.starts_with("*") {
            return Err(winnow::error::ContextError::new());
        }
    }

    let start = parser_input.input.current_token_start();
    literal("*").parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Text(TextElement {
        location: Location { start, end },
        content: "*".to_string(),
    }))
}
