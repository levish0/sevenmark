use super::escape::escape_parser;
use super::text::text_parser;
use super::token::*;
use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::{InputSource, ParserInput};
use winnow::Result;
use winnow::combinator::alt;
use winnow::combinator::repeat;
use winnow::prelude::*;
use super::markdown::{markdown_bolditalic_parser, markdown_bold_parser, markdown_italic_parser, markdown_hline_parser, markdown_strikethrough_parser, markdown_header_parser};

/// 요소 파서 - 다양한 SevenMark 요소들을 파싱
pub fn element_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    // println!("{:?}", parser_input.state);
    // println!("{:?}", parser_input.input);

    let result = repeat(
        1..,
        alt((
            escape_parser,
            markdown_header_parser,
            markdown_hline_parser,
            markdown_bolditalic_parser,
            markdown_bold_parser,
            markdown_italic_parser,
            markdown_strikethrough_parser,
            text_parser,

            token_newline_parser,
            token_brace_open_parser,
            token_brace_close_parser,
            token_bracket_open_parser,
            token_bracket_close_parser,
            token_slash_parser,
            token_asterisk_parser,
            token_underscore_parser,
            token_tilde_parser,
            token_caret_parser,
            token_comma_parser,
        )),
    )
    .map(|elements: Vec<_>| elements.into_iter().flatten().collect())
    .parse_next(parser_input);

    result
}
