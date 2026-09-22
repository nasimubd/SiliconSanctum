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

pub struct RadixMarker;
