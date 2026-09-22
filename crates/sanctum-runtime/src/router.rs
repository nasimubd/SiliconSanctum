//! Concurrent speculative fan-out routing.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq)]
pub enum RouterError {
    EmptyPrompt,
    ProbabilityOutOfRange,
    InvalidThresholds,
    EmptyRouteTarget,
}

pub struct RouterMarker;
