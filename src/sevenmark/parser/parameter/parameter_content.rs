use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::parser::escape::escape::escape_parser;
use crate::sevenmark::parser::parameter::parameter_text::parameter_text_parser;
use winnow::Result;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

/// Parse the content within parameter value quotes
/// Handles both escape sequences and plain text content
/// The literal syntax prioritizes: escaping and text parsing
pub fn parameter_content_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    repeat(1.., alt((escape_parser, parameter_text_parser))).parse_next(parser_input)
}
