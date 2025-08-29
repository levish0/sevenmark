use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::literal;

pub fn macro_newline_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    literal("[br]").parse_next(parser_input)?;

    Ok(SevenMarkElement::NewLine)
}
