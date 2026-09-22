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
