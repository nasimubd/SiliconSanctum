//! Adaptive speculative-decoding primitives.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeculativeError {
    ZeroValue(&'static str),
    DraftModelTooLarge(u64),
    TargetModelTooSmall(u64),
    ContextPositionOutOfRange { position: u32, capacity: u32 },
    InvalidWidthBounds { minimum: u8, maximum: u8 },
    EmptyProposal,
    ContextExhausted,
    Cancelled,
    Backend(String),
    IllegalTransition,
}

// NEXT
