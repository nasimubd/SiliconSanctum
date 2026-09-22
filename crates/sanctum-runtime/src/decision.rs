//! Bounded System One decision runtime.
#![allow(clippy::missing_errors_doc)]

pub const MIN_DECISION_LATENCY: std::time::Duration = std::time::Duration::from_millis(70);

pub const MAX_DECISION_LATENCY: std::time::Duration = std::time::Duration::from_millis(500);

#[derive(Debug, Clone, PartialEq)]
pub enum DecisionError {
    LatencyBelowMinimum,
    LatencyAboveMaximum,
    InvertedLatencyRange,
    EmptyField { field: &'static str },
    ZeroParameterCount,
    ParameterLimitExceeded { parameters: u64 },
    ZeroDimension,
    TensorSizeOverflow,
    EmptyInput,
    NonFiniteInput,
    EmptyLogits,
    NonFiniteLogit,
    ProbabilityOutOfRange,
    EmptyChoices,
    TooManyChoices,
    DuplicateChoice(String),
    InvalidRubric,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LatencyBudget {
    pub minimum: std::time::Duration,
    pub maximum: std::time::Duration,
}

pub struct DecisionMarker;
