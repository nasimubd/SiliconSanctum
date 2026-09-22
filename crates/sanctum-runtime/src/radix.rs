//! Token radix prefix caching.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadixError {
    EmptyTokens,
    EmptySession,
    ZeroCapacity,
    UnsupportedAttention,
    MissingSession,
}

pub type TokenId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenSequence(Vec<TokenId>);

impl TokenSequence {
    pub fn new(tokens: Vec<TokenId>) -> Result<Self, RadixError> {
        if tokens.is_empty() {
            return Err(RadixError::EmptyTokens);
        }
        Ok(Self(tokens))
    }
}

impl TokenSequence {
    #[must_use]
    pub fn tokens(&self) -> &[TokenId] {
        &self.0
    }
}

impl TokenSequence {
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl TokenSequence {
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionId(String);

impl SessionId {
    pub fn new(v: impl Into<String>) -> Result<Self, RadixError> {
        let v = v.into();
        if v.trim().is_empty() {
            return Err(RadixError::EmptySession);
        }
        Ok(Self(v))
    }
}

impl SessionId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttentionArchitecture {
    Full,
    SlidingWindow,
    Recurrent,
    Hybrid,
}

impl AttentionArchitecture {
    #[must_use]
    pub const fn supports_prefix_cache(self) -> bool {
        matches!(self, Self::Full)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelArchitecture {
    pub name: String,
    pub attention: AttentionArchitecture,
}

impl ModelArchitecture {
    pub fn validate_prefix_cache(&self) -> Result<(), RadixError> {
        if self.attention.supports_prefix_cache() {
            Ok(())
        } else {
            Err(RadixError::UnsupportedAttention)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheCapacity {
    pub max_nodes: usize,
    pub max_bytes: usize,
}

impl CacheCapacity {
    pub fn new(max_nodes: usize, max_bytes: usize) -> Result<Self, RadixError> {
        if max_nodes == 0 || max_bytes == 0 {
            return Err(RadixError::ZeroCapacity);
        }
        Ok(Self {
            max_nodes,
            max_bytes,
        })
    }
}

impl Default for CacheCapacity {
    fn default() -> Self {
        Self {
            max_nodes: 65_536,
            max_bytes: 512 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CacheHandle(pub u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrefixMatch {
    pub handle: Option<CacheHandle>,
    pub matched_tokens: usize,
    pub remaining_tokens: usize,
}

impl PrefixMatch {
    #[must_use]
    pub const fn hit(&self) -> bool {
        self.matched_tokens > 0
    }
}

#[derive(Debug, Default)]
pub struct RadixNode {
    children: std::collections::BTreeMap<TokenId, RadixNode>,
    handle: Option<CacheHandle>,
    bytes: usize,
    last_used: u64,
}

impl RadixNode {
    #[must_use]
    pub fn child_count(&self) -> usize {
        self.children.len()
    }
}

impl RadixNode {
    #[must_use]
    pub const fn handle(&self) -> Option<CacheHandle> {
        self.handle
    }
}

impl RadixNode {
    #[must_use]
    pub const fn stored_bytes(&self) -> usize {
        self.bytes
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CacheStats {
    pub nodes: usize,
    pub bytes: usize,
    pub entries: usize,
    pub hits: u64,
    pub misses: u64,
}

#[derive(Debug)]
pub struct RadixCache {
    root: RadixNode,
    capacity: CacheCapacity,
    stats: CacheStats,
    next_handle: u64,
    clock: u64,
    sessions: std::collections::HashMap<SessionId, TokenSequence>,
}

impl RadixCache {
    #[must_use]
    pub fn new(capacity: CacheCapacity) -> Self {
        Self {
            root: RadixNode::default(),
            capacity,
            stats: CacheStats {
                nodes: 1,
                ..CacheStats::default()
            },
            next_handle: 1,
            clock: 0,
            sessions: std::collections::HashMap::new(),
        }
    }
}

impl RadixCache {
    #[must_use]
    pub const fn capacity(&self) -> CacheCapacity {
        self.capacity
    }
}

pub struct RadixMarker;
