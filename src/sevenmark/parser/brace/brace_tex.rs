use crate::sevenmark::ast::SevenMarkElement;
use crate::sevenmark::{Location, ParserInput, TeXElement};
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_until};

/// Parse TeX elements enclosed in {{{#tex }}}
pub fn brace_tex_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let parsed_content = delimited(literal("{{{#tex"), take_until(0.., "}}}"), literal("}}}"))
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();

    Ok(SevenMarkElement::TeXElement(TeXElement {
        location: Location { start, end },
        content: parsed_content.to_string(),
    }))
}
