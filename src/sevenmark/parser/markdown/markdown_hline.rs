use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::ParserInput;
use winnow::ascii::line_ending;
use winnow::combinator::eof;
use winnow::combinator::{alt, terminated};
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Result;
pub fn markdown_hline_parser(input: &mut ParserInput) -> Result<SevenMarkElement> {
    terminated(take_while(3..=9, '-'), alt((line_ending, eof))).parse_next(input)?;

    Ok(SevenMarkElement::HLine)
}
