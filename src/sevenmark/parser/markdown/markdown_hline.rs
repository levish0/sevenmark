use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::{Location, SevenMarkElement};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::eof;
use winnow::combinator::{alt, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::take_while;
pub fn markdown_hline_parser(input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = input.input.current_token_start() + input.state.base_offset;

    terminated(take_while(3..=9, '-'), alt((line_ending, eof))).parse_next(input)?;

    let end = input.input.previous_token_end() + input.state.base_offset;

    Ok(SevenMarkElement::HLine)
}
