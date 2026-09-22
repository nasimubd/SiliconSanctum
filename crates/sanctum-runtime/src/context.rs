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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceLanguage {
    Python,
    Rust,
    Cpp,
    TypeScript,
}

pub struct ContextMarker;
