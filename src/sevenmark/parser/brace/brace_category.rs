use crate::sevenmark::ast::{Location, SevenMarkElement};
use crate::sevenmark::parser::brace::category::category_content_parser;
use crate::sevenmark::{CategoryElement, ParserInput};
use winnow::Result;
use winnow::ascii::multispace0;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::literal;

pub fn brace_category_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (_, parsed_content) = delimited(
        literal("{{{#category"),
        (multispace0, |input: &mut ParserInput| {
            let mut inner_input = input.clone();
            inner_input
                .state
                .increase_depth()
                .map_err(|e| e.into_context_error())?;
            let result = category_content_parser(&mut inner_input);
            inner_input
                .state
                .decrease_depth()
                .map_err(|e| e.into_context_error())?;
            *input = inner_input;
            result
        }),
        literal("}}}"),
    )
    .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::Category(CategoryElement {
        location: Location { start, end },
        content: parsed_content,
    }))
}
