use crate::sevenmark::context::ParseContext;
use winnow::Result;
use winnow::stream::{LocatingSlice, Stateful};

pub mod document;
pub mod element;
pub mod escape;
pub mod markdown;
mod parameter;
pub mod text;
pub mod token;
mod comment;
mod brace;

pub use document::parse_document;

pub type InputSource<'i> = LocatingSlice<&'i str>;

pub type ParserInput<'i> = Stateful<InputSource<'i>, ParseContext>;
