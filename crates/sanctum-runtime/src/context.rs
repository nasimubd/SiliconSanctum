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

impl SourceLanguage {
    pub fn from_extension(v: &str) -> Result<Self, ContextError> {
        match v.trim_start_matches('.').to_ascii_lowercase().as_str() {
            "py" => Ok(Self::Python),
            "rs" => Ok(Self::Rust),
            "cc" | "cpp" | "cxx" | "hpp" => Ok(Self::Cpp),
            "ts" | "tsx" => Ok(Self::TypeScript),
            other => Err(ContextError::UnsupportedExtension(other.to_owned())),
        }
    }
}

impl SourceLanguage {
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Python => "python",
            Self::Rust => "rust",
            Self::Cpp => "cpp",
            Self::TypeScript => "typescript",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceDocument {
    path: std::path::PathBuf,
    language: SourceLanguage,
    source: Vec<u8>,
}

impl SourceDocument {
    pub fn new(path: impl Into<std::path::PathBuf>, source: Vec<u8>) -> Result<Self, ContextError> {
        let path = path.into();
        if path.as_os_str().is_empty() {
            return Err(ContextError::EmptyPath);
        }
        if source.is_empty() {
            return Err(ContextError::EmptySource);
        }
        let ext = path
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .ok_or_else(|| ContextError::UnsupportedExtension(String::new()))?;
        let language = SourceLanguage::from_extension(ext)?;
        Ok(Self {
            path,
            language,
            source,
        })
    }
}

impl SourceDocument {
    #[must_use]
    pub const fn language(&self) -> SourceLanguage {
        self.language
    }
}

impl SourceDocument {
    #[must_use]
    pub fn path(&self) -> &std::path::Path {
        &self.path
    }
}

impl SourceDocument {
    #[must_use]
    pub fn source(&self) -> &[u8] {
        &self.source
    }
}

#[must_use]
pub fn grammar(language: SourceLanguage) -> tree_sitter::Language {
    match language {
        SourceLanguage::Python => tree_sitter_python::LANGUAGE.into(),
        SourceLanguage::Rust => tree_sitter_rust::LANGUAGE.into(),
        SourceLanguage::Cpp => tree_sitter_cpp::LANGUAGE.into(),
        SourceLanguage::TypeScript => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
    }
}

pub fn parser_for(language: SourceLanguage) -> Result<tree_sitter::Parser, ContextError> {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&grammar(language))
        .map_err(|_| ContextError::ParserConfiguration)?;
    Ok(parser)
}

pub struct ParsedDocument {
    document: SourceDocument,
    tree: tree_sitter::Tree,
}

impl ParsedDocument {
    pub fn parse(document: SourceDocument) -> Result<Self, ContextError> {
        let mut parser = parser_for(document.language())?;
        let tree = parser
            .parse(document.source(), None)
            .ok_or(ContextError::ParseFailed)?;
        Ok(Self { document, tree })
    }
}

impl ParsedDocument {
    #[must_use]
    pub fn root(&self) -> tree_sitter::Node<'_> {
        self.tree.root_node()
    }
}

impl ParsedDocument {
    #[must_use]
    pub fn sexp(&self) -> String {
        self.root().to_sexp()
    }
}

impl ParsedDocument {
    #[must_use]
    pub fn source(&self) -> &[u8] {
        self.document.source()
    }
}

impl ParsedDocument {
    #[must_use]
    pub const fn language(&self) -> SourceLanguage {
        self.document.language
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceRange {
    pub start: usize,
    pub end: usize,
}

impl SourceRange {
    pub fn new(start: usize, end: usize) -> Result<Self, ContextError> {
        if start > end {
            return Err(ContextError::InvalidRange);
        }
        Ok(Self { start, end })
    }
}

impl SourceRange {
    #[must_use]
    pub const fn len(self) -> usize {
        self.end - self.start
    }
}

impl SourceRange {
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }
}

pub struct ContextMarker;
