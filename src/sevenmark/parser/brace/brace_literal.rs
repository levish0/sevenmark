use crate::sevenmark::ast::{LiteralElement, Location, SevenMarkElement};
use crate::sevenmark::ParserInput;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;
use super::literal::literal_content_parser;

/// Parse literal elements enclosed in {{{ }}}
pub fn brace_literal_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start() + parser_input.state.base_offset;
    
    let content = delimited(
        literal("{{{"),
        |input: &mut ParserInput| {
            let mut modified_input = *input;
            modified_input.state = input.state.with_offset(start + 3)
                .map_err(|e| e.into_context_error())?;
            let result = literal_content_parser(&mut modified_input);
            
            // Copy the modified input state back to the original input
            *input = modified_input;
            
            result
        },
        literal("}}}"),
    ).parse_next(parser_input)?;
    
    let end = parser_input.input.previous_token_end() + parser_input.state.base_offset;

    Ok(SevenMarkElement::Literal(LiteralElement {
        location: Location { start, end },
        content,
    }))
}