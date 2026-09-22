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

pub struct ContextMarker;
