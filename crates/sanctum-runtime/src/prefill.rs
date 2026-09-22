//! Chunked context prefill scheduling.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefillError {
    EmptyRequest,
    ZeroBudget,
    InvalidGeometry,
    InvalidThresholds,
    UnsafeScratch,
    InsufficientMemory,
    CriticalPressure,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TokenCount(usize);

impl TokenCount {
    pub fn new(value: usize) -> Result<Self, PrefillError> {
        if value == 0 {
            return Err(PrefillError::EmptyRequest);
        }
        Ok(Self(value))
    }

    #[must_use]
    pub const fn get(self) -> usize {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefillStep {
    Tokens512,
    Tokens1024,
}

impl PrefillStep {
    #[must_use]
    pub const fn tokens(self) -> usize {
        match self {
            Self::Tokens512 => 512,
            Self::Tokens1024 => 1024,
        }
    }
}
