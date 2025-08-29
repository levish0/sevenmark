use crate::sevenmark::ParserInput;
use crate::sevenmark::ast::SevenMarkElement;
use winnow::Result;
use winnow::combinator::delimited;
use winnow::prelude::*;
use winnow::token::literal;
use winnow::token::take_while;

pub fn macro_age_parser(parser_input: &mut ParserInput) -> Result<SevenMarkElement> {
    let date =
        delimited(literal("[age("), utils_parse_date, literal(")]")).parse_next(parser_input)?;

    Ok(SevenMarkElement::Age(date))
}

// ISO 8601
fn utils_parse_date(parser_input: &mut ParserInput) -> Result<String> {
    let year = take_while(4..=4, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;
    literal("-").parse_next(parser_input)?;
    let month = take_while(2..=2, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;
    literal("-").parse_next(parser_input)?;
    let day = take_while(2..=2, |c: char| c.is_ascii_digit()).parse_next(parser_input)?;

    Ok(format!("{}-{}-{}", year, month, day))
}
