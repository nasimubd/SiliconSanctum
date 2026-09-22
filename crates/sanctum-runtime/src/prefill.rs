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

    #[must_use]
    pub const fn for_tokens(tokens: TokenCount) -> Self {
        if tokens.get() <= 512 {
            Self::Tokens512
        } else {
            Self::Tokens1024
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScratchBudget(u64);

impl ScratchBudget {
    pub const fn new(bytes: u64) -> Result<Self, PrefillError> {
        if bytes == 0 {
            return Err(PrefillError::ZeroBudget);
        }
        Ok(Self(bytes))
    }

    #[must_use]
    pub const fn bytes(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub total_bytes: u64,
    pub resident_bytes: u64,
    pub reserved_bytes: u64,
}
