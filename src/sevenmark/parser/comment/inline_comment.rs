use crate::sevenmark::ast::{CommentElement, Location, SevenMarkElement};
use crate::sevenmark::ParserInput;
use winnow::ascii::line_ending;
use winnow::combinator::{alt, eof, opt, terminated};
use winnow::prelude::*;
use winnow::stream::Location as StreamLocation;
use winnow::token::{literal, take_till};
use winnow::Result;

/// Parse inline comments starting with "//"
/// Comments continue until end of line or end of file
pub fn inline_comment_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let start = parser_input.input.current_token_start();

    let (_, content_opt) = (
        literal("//"),
        terminated(
            opt(take_till(0.., |c: char| c == '\n')),
            alt((line_ending, eof)),
        ),
    )
        .parse_next(parser_input)?;

    let end = parser_input.input.previous_token_end();
    let content = content_opt.unwrap_or("").to_string();

    Ok(SevenMarkElement::Comment(CommentElement {
        location: Location { start, end },
        content,
    }))
}
