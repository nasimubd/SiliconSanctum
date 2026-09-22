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

pub struct RadixMarker;
