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

pub struct RadixMarker;
