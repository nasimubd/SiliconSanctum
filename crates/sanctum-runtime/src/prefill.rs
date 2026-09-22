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

impl MemorySnapshot {
    #[must_use]
    pub const fn available_bytes(self) -> u64 {
        self.total_bytes
            .saturating_sub(self.resident_bytes)
            .saturating_sub(self.reserved_bytes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KvQuantization {
    Bits4,
    Bits8,
    Bits16,
}

impl KvQuantization {
    #[must_use]
    pub const fn bits(self) -> u8 {
        match self {
            Self::Bits4 => 4,
            Self::Bits8 => 8,
            Self::Bits16 => 16,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelGeometry {
    pub layers: u32,
    pub kv_heads: u32,
    pub head_dim: u32,
}

impl ModelGeometry {
    pub const fn new(layers: u32, kv_heads: u32, head_dim: u32) -> Result<Self, PrefillError> {
        if layers == 0 || kv_heads == 0 || head_dim == 0 {
            return Err(PrefillError::InvalidGeometry);
        }
        Ok(Self {
            layers,
            kv_heads,
            head_dim,
        })
    }
}
