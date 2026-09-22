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

pub struct RadixMarker;
