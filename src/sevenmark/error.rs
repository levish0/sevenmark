use std::fmt;
use winnow::error::ContextError;

#[derive(Debug, Clone, PartialEq)]
pub enum SevenMarkError {
    RecursionDepthExceeded { depth: usize, max_depth: usize },
}

impl fmt::Display for SevenMarkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SevenMarkError::RecursionDepthExceeded { depth, max_depth } => {
                write!(f, "Recursion depth exceeded: {} > {}", depth, max_depth)
            }
        }
    }
}

impl std::error::Error for SevenMarkError {}

impl SevenMarkError {
    /// SevenMarkError를 winnow::error::ContextError로 변환
    pub fn into_context_error(self) -> ContextError {
        ContextError::new()
    }
}
