use crate::sevenmark::ast::{LiteralElement, Location, SevenMarkElement};
use crate::sevenmark::ParserInput;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::{Location as StreamLocation, Stream};
use winnow::token::literal;
use super::literal::literal_content_parser;

/// Parse literal elements enclosed in {{{ }}}
pub fn brace_literal_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();
    
    let content = delimited(
        literal("{{{"),
        |input: &mut ParserInput| {
            let mut inner_input = input.clone();
            inner_input.state.increase_depth().map_err(|e| e.into_context_error())?;
            let result = literal_content_parser(&mut inner_input);
            inner_input.state.decrease_depth().map_err(|e| e.into_context_error())?;
            *input = inner_input;
            result
        },
        literal("}}}"),
    ).parse_next(parser_input)?;
    
    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Literal(LiteralElement {
        location: Location { start, end },
        content,
    }))
}