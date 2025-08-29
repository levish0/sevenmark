use serde::Serialize;
use std::collections::HashMap;

/// 파라미터 맵: key-value 쌍으로 각 value는 SevenMarkElement> 벡터
pub type Parameters = HashMap<String, Vec<SevenMarkElement>>;

/// 소스 코드 위치 정보
#[derive(Debug, Clone, Serialize)]
pub struct Location {
    pub start: usize,
    pub end: usize,
}

/// 텍스트 요소
#[derive(Debug, Clone, Serialize)]
pub struct TextElement {
    pub location: Location,
    pub content: String,
}

/// 이스케이프 요소
#[derive(Debug, Clone, Serialize)]
pub struct EscapeElement {
    pub location: Location,
    pub content: String,
}

/// 주석 요소
#[derive(Debug, Clone, Serialize)]
pub struct CommentElement {
    pub location: Location,
    pub content: String,
}

/// 공통 스타일 속성들
#[derive(Debug, Clone, Serialize)]
pub struct CommonStyleAttributes {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub style: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub size: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub color: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bg_color: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub opacity: Vec<SevenMarkElement>,
}

/// 리터럴 요소 {{{ content }}}
#[derive(Debug, Clone, Serialize)]
pub struct LiteralElement {
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 스타일이 적용된 요소 {{{#style="..." content}}}
#[derive(Debug, Clone, Serialize)]
pub struct StyledElement {
    pub location: Location,
    pub common_style: CommonStyleAttributes,
    pub content: Vec<SevenMarkElement>,
}

/// 미디어 요소 [[#file="..." #url="..." display_text]]
#[derive(Debug, Clone, Serialize)]
pub struct MediaElement {
    pub location: Location,
    pub file: Vec<SevenMarkElement>,
    pub url: Vec<SevenMarkElement>,
    pub display_text: Vec<SevenMarkElement>,
}

/// 테이블 셀
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement2 {
    pub common_style: CommonStyleAttributes,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub x: Vec<SevenMarkElement>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub y: Vec<SevenMarkElement>,
    pub content: Vec<SevenMarkElement>,
}

/// 테이블 행
#[derive(Debug, Clone, Serialize)]
pub struct TableInnerElement1 {
    pub common_style: CommonStyleAttributes,
    pub inner_content: Vec<TableInnerElement2>,
}

/// 테이블 요소
#[derive(Debug, Clone, Serialize)]
pub struct TableElement {
    pub location: Location,
    pub common_style: CommonStyleAttributes,
    pub content: Vec<TableInnerElement1>,
}

/// 리스트 아이템
#[derive(Debug, Clone, Serialize)]
pub struct ListInnerElement1 {
    pub common_style: CommonStyleAttributes,
    pub content: Vec<SevenMarkElement>,
}

/// 리스트 요소
#[derive(Debug, Clone, Serialize)]
pub struct ListElement {
    pub kind: String,
    pub common_style: CommonStyleAttributes,
    pub content: Vec<ListInnerElement1>,
}

/// 폴드 내부 요소
#[derive(Debug, Clone, Serialize)]
pub struct FoldInnerElement {
    pub common_style: CommonStyleAttributes,
    pub content: Vec<SevenMarkElement>,
}

/// 폴드 요소
#[derive(Debug, Clone, Serialize)]
pub struct FoldElement {
    pub common_style: CommonStyleAttributes,
    pub content: (FoldInnerElement, FoldInnerElement),
}

/// 인용 블록
#[derive(Debug, Clone, Serialize)]
pub struct BlockQuoteElement {
    pub common_style: CommonStyleAttributes,
    pub content: Vec<SevenMarkElement>,
}

/// 루비 텍스트
#[derive(Debug, Clone, Serialize)]
pub struct RubyElement {
    pub base: Vec<SevenMarkElement>,
    pub ruby: Vec<SevenMarkElement>,
}

/// 코드 블록
#[derive(Debug, Clone, Serialize)]
pub struct CodeElement {
    pub location: Location,
    pub language: Vec<SevenMarkElement>,
    pub content: Vec<SevenMarkElement>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TeXElement {
    pub location: Location,
    pub content: String,
}

/// 각주
#[derive(Debug, Clone, Serialize)]
pub struct FootnoteElement {
    pub content: Vec<SevenMarkElement>,
}

/// 헤더
#[derive(Debug, Clone, Serialize)]
pub struct Header {
    pub location: Location,
    pub level: usize,
    pub is_folded: bool,
    pub content: Vec<SevenMarkElement>,
}

/// 텍스트 스타일 (Bold, Italic 등)
#[derive(Debug, Clone, Serialize)]
pub struct TextStyle {
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 포함 요소
#[derive(Debug, Clone, Serialize)]
pub struct IncludeElement {
    pub location: Location,
    pub parameters: Parameters,
    pub content: Vec<SevenMarkElement>,
}

/// 카테고리 요소
#[derive(Debug, Clone, Serialize)]
pub struct CategoryElement {
    pub location: Location,
    pub content: Vec<SevenMarkElement>,
}

/// 리다이렉트 요소
#[derive(Debug, Clone, Serialize)]
pub struct RedirectElement {
    pub content: Vec<SevenMarkElement>,
}

/// 메인 SevenMark AST 요소들
#[derive(Debug, Clone, Serialize)]
pub enum SevenMarkElement {
    // Basic text elements
    Text(TextElement),
    Comment(CommentElement),
    Escape(EscapeElement),

    // Block elements
    LiteralElement(LiteralElement),
    StyledElement(StyledElement),
    TableElement(TableElement),
    ListElement(ListElement),
    FoldElement(FoldElement),
    BlockQuoteElement(BlockQuoteElement),
    RubyElement(RubyElement),
    FootnoteElement(FootnoteElement),
    CodeElement(CodeElement),
    TeXElement(TeXElement),

    // Wiki elements
    Include(IncludeElement),
    Category(CategoryElement),
    Redirect(RedirectElement),

    // Media
    MediaElement(MediaElement),

    // Macros
    Null,
    FootNote,
    TimeNow,
    Age(String),
    Variable(String),

    // Markdown text styles
    BoldItalic(TextStyle),
    Bold(TextStyle),
    Italic(TextStyle),
    Strikethrough(TextStyle),
    Underline(TextStyle),
    Superscript(TextStyle),
    Subscript(TextStyle),

    // Other markdown elements
    HLine,
    Header(Header),
}

impl Default for CommonStyleAttributes {
    fn default() -> Self {
        Self {
            style: Vec::new(),
            size: Vec::new(),
            color: Vec::new(),
            bg_color: Vec::new(),
            opacity: Vec::new(),
        }
    }
}
