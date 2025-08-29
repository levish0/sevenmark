use crate::sevenmark::context::ParseContext;
use winnow::stream::{LocatingSlice, Stateful};

mod brace;
mod comment;
pub mod document;
pub mod element;
pub mod escape;
pub mod markdown;
mod parameter;
pub mod text;
pub mod token;
mod utils;
mod r#macro;

pub use document::parse_document;

pub type InputSource<'i> = LocatingSlice<&'i str>;

pub type ParserInput<'i> = Stateful<InputSource<'i>, ParseContext>;
