//! Multi-language structural context extraction.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextError {
    UnsupportedExtension(String),
    EmptyPath,
    EmptySource,
    ParserConfiguration,
    ParseFailed,
    InvalidRange,
    EmptyQuery,
    QueryCompilation,
}

pub struct ContextMarker;
