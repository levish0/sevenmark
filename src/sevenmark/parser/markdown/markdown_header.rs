use super::super::element::element_parser;
use crate::sevenmark::ast::{Header, Location, SevenMarkElement};
use crate::sevenmark::{InputSource, ParserInput};
use winnow::Result;
use winnow::ascii::line_ending;
use winnow::combinator::eof;
use winnow::combinator::{alt, opt, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_till, take_while};

/// 헤더 파서 - # Header (1-6개의 # 지원, ! 폴딩 지원)  
pub fn markdown_header_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start() + parser_input.state.base_offset;

    let (header_marks, is_folded, (_, content)) = (
        take_while(1..=6, '#'),
        opt(literal('!')),
        terminated(
            (opt(literal(' ')), take_till(0.., '\n')),
            alt((line_ending, eof)),
        ),
    )
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end() + parser_input.state.base_offset;
    let header_level = header_marks.len();
    let is_folded = is_folded.is_some();

    let new_context = parser_input
        .state
        .with_offset(start + header_level + if is_folded { 1 } else { 0 } + 1)
        .map_err(|e| e.into_context_error())?;

    let mut nested_parser = ParserInput {
        input: InputSource::new(content),
        state: new_context,
    };

    let parsed_content = element_parser(&mut nested_parser)?;

    Ok(SevenMarkElement::Header(Header {
        location: Location { start, end },
        level: header_level,
        is_folded,
        content: parsed_content,
    }))
}
