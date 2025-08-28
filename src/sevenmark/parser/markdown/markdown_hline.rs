use crate::sevenmark::ast::{Location, SevenMarkElement};
use crate::sevenmark::ParserInput;
use winnow::Result;
use winnow::combinator::{alt, terminated};
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::ascii::line_ending;
use winnow::combinator::eof;
use winnow::stream::Location as StreamLocation;
pub fn markdown_hline_parser(input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    let start = input.input.current_token_start() + input.state.base_offset;
    
    terminated(
        take_while(3.., '-'),
        alt((line_ending, eof))
    ).parse_next(input)?;
    
    let end = input.input.previous_token_end() + input.state.base_offset;

    Ok(vec![SevenMarkElement::HLine])
}