use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;

use super::code_text::code_text_parser;
use crate::sevenmark::parser::escape::escape::escape_parser;
use crate::sevenmark::parser::token::{token_brace_close_parser, token_brace_open_parser};
use winnow::Result;
use winnow::combinator::{alt, repeat};
use winnow::prelude::*;

pub fn code_content_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    repeat(
        1..,
        alt((
            escape_parser,
            // code text
            code_text_parser,
            // Token
            token_brace_open_parser,
            token_brace_close_parser,
        )),
    )
    .parse_next(parser_input)
}
