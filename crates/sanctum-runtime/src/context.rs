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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Class,
    Function,
    Interface,
    Type,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructuralSymbol {
    pub kind: SymbolKind,
    pub name: String,
    pub signature: String,
    pub return_type: Option<String>,
    pub documentation: Option<String>,
    pub range: SourceRange,
}

impl StructuralSymbol {
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl StructuralSymbol {
    #[must_use]
    pub fn signature(&self) -> &str {
        &self.signature
    }
}

impl StructuralSymbol {
    #[must_use]
    pub fn return_type(&self) -> Option<&str> {
        self.return_type.as_deref()
    }
}

impl StructuralSymbol {
    #[must_use]
    pub fn documentation(&self) -> Option<&str> {
        self.documentation.as_deref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuerySpec {
    source: String,
}

impl QuerySpec {
    pub fn new(source: impl Into<String>) -> Result<Self, ContextError> {
        let source = source.into();
        if source.trim().is_empty() {
            return Err(ContextError::EmptyQuery);
        }
        Ok(Self { source })
    }
}

impl QuerySpec {
    #[must_use]
    pub fn source(&self) -> &str {
        &self.source
    }
}

pub fn compile_query(
    language: SourceLanguage,
    spec: &QuerySpec,
) -> Result<tree_sitter::Query, ContextError> {
    tree_sitter::Query::new(&grammar(language), spec.source())
        .map_err(|_| ContextError::QueryCompilation)
}

#[must_use]
pub fn python_query() -> QuerySpec {
    QuerySpec { source: "[(class_definition name: (identifier) @name) (function_definition name: (identifier) @name)] @definition".to_owned() }
}

#[must_use]
pub fn rust_query() -> QuerySpec {
    QuerySpec { source: "[(struct_item name: (type_identifier) @name) (enum_item name: (type_identifier) @name) (function_item name: (identifier) @name)] @definition".to_owned() }
}

#[must_use]
pub fn cpp_query() -> QuerySpec {
    QuerySpec {
        source:
            "[(class_specifier name: (type_identifier) @name) (function_definition) @definition]"
                .to_owned(),
    }
}

#[must_use]
pub fn typescript_query() -> QuerySpec {
    QuerySpec { source: "[(class_declaration name: (type_identifier) @name) (function_declaration name: (identifier) @name) (interface_declaration name: (type_identifier) @name)] @definition".to_owned() }
}

#[must_use]
pub fn query_for(language: SourceLanguage) -> QuerySpec {
    match language {
        SourceLanguage::Python => python_query(),
        SourceLanguage::Rust => rust_query(),
        SourceLanguage::Cpp => cpp_query(),
        SourceLanguage::TypeScript => typescript_query(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryCapture {
    pub name: String,
    pub range: SourceRange,
    pub text: String,
}

impl QueryCapture {
    #[must_use]
    pub fn start_byte(&self) -> usize {
        self.range.start
    }
}

#[must_use]
pub fn symbol_kind(language: SourceLanguage, node_kind: &str) -> Option<SymbolKind> {
    match (language, node_kind) {
        (_, "class_definition" | "class_specifier" | "class_declaration") => {
            Some(SymbolKind::Class)
        }
        (SourceLanguage::TypeScript, "interface_declaration") => Some(SymbolKind::Interface),
        (
            _,
            "function_definition" | "function_item" | "function_declaration" | "method_definition",
        ) => Some(SymbolKind::Function),
        (SourceLanguage::Rust, "struct_item" | "enum_item") => Some(SymbolKind::Type),
        _ => None,
    }
}

#[must_use]
pub fn body_field(_language: SourceLanguage) -> &'static str {
    "body"
}

#[must_use]
pub fn node_text<'a>(node: tree_sitter::Node<'_>, source: &'a [u8]) -> Option<&'a str> {
    node.utf8_text(source).ok()
}

pub fn node_range(node: tree_sitter::Node<'_>) -> Result<SourceRange, ContextError> {
    SourceRange::new(node.start_byte(), node.end_byte())
}

#[must_use]
pub fn node_name(node: tree_sitter::Node<'_>, source: &[u8]) -> String {
    node.child_by_field_name("name")
        .and_then(|v| node_text(v, source))
        .unwrap_or(node.kind())
        .to_owned()
}

#[must_use]
pub fn signature_end(node: tree_sitter::Node<'_>, language: SourceLanguage) -> usize {
    node.child_by_field_name(body_field(language))
        .map_or(node.end_byte(), |body| body.start_byte())
}

#[must_use]
pub fn signature_text(
    node: tree_sitter::Node<'_>,
    language: SourceLanguage,
    source: &[u8],
) -> String {
    let end = signature_end(node, language).min(source.len());
    String::from_utf8_lossy(&source[node.start_byte()..end])
        .trim()
        .to_owned()
}

fn collect_nodes(
    node: tree_sitter::Node<'_>,
    language: SourceLanguage,
    source: &[u8],
    output: &mut Vec<StructuralSymbol>,
) {
    if let Some(kind) = symbol_kind(language, node.kind()) {
        if let Ok(range) = node_range(node) {
            output.push(StructuralSymbol {
                kind,
                name: node_name(node, source),
                signature: signature_text(node, language, source),
                return_type: None,
                documentation: None,
                range,
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_nodes(child, language, source, output);
    }
}

pub struct ContextMarker;
