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
}
