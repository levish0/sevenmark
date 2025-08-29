use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::eof;
use winnow::combinator::{alt, terminated};
use winnow::prelude::*;
use winnow::token::take_while;
pub fn markdown_hline_parser(input: &mut ParserInput) -> Result<SevenMarkElement> {
    terminated(take_while(3..=9, '-'), alt((line_ending, eof))).parse_next(input)?;

    Ok(SevenMarkElement::HLine)
}
