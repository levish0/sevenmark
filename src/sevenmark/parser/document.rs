use super::element::element_parser;
use super::{InputSource, ParserInput};
use crate::sevenmark::ast::{SevenMarkElement, TextElement};
use crate::sevenmark::context::ParseContext;
use winnow::combinator::repeat;
use winnow::prelude::*;
use winnow::stream::Location;
use winnow::Result;

/// 메인 문서 파서 - 전체 입력을 파싱
pub fn parse_document(input: &str) -> Vec<SevenMarkElement> {
    let normalized_input = input.replace("\r\n", "\n");

    let context = ParseContext::new();
    let mut stateful_input = ParserInput {
        input: InputSource::new(&normalized_input),
        state: context,
    };

    match document_parser(&mut stateful_input) {
        Ok(mut elements) => {
            // 파싱하고 남은 부분이 있다면, 그 부분도 Text 엘리먼트로 만들어 추가합니다.
            if !stateful_input.input.is_empty() {
                let start = stateful_input.input.current_token_start();
                let remaining = stateful_input.input.to_string();
                let end = start + remaining.len();

                elements.push(SevenMarkElement::Text(TextElement {
                    location: crate::sevenmark::ast::Location { start, end },
                    content: remaining,
                }));
            }
            elements
        }
        Err(_) => {
            // 파서가 처음부터 실패했다면, 입력 전체를 하나의 Text 엘리먼트로 처리합니다.
            vec![SevenMarkElement::Text(TextElement {
                location: crate::sevenmark::ast::Location {
                    start: 0,
                    end: input.len(),
                },
                content: input.to_string(),
            })]
        }
    }
}

/// 문서 파서 - element들을 반복 파싱 (기존 many0 + alt 패턴)
fn document_parser(parser_input: &mut ParserInput) -> Result<Vec<SevenMarkElement>> {
    repeat(0.., element_parser)
        .map(|elements: Vec<_>| elements.into_iter().flatten().collect())
        .parse_next(parser_input)
}
