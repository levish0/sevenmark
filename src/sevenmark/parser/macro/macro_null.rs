use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use winnow::Result;
use winnow::prelude::*;
use winnow::token::literal;

/// Parse null macro [null] -> returns Null element
pub fn macro_null_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    literal("[null]").parse_next(parser_input)?;

    Ok(SevenMarkElement::Null)
}
