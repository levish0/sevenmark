use crate::sevenmark::ast::{EscapeElement, Location, SevenMarkElement};
use crate::sevenmark::ParserInput;
use winnow::combinator::preceded;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{any, literal};
use winnow::Result;

pub fn escape_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();
    let parsed_content = preceded(literal("\\"), any).parse_next(parser_input)?;
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Escape(EscapeElement {
        location: Location { start, end },
        content: parsed_content.to_string(),
    }))
}
