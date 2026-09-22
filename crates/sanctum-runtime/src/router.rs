//! Concurrent speculative fan-out routing.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq)]
pub enum RouterError {
    EmptyPrompt,
    ProbabilityOutOfRange,
    InvalidThresholds,
    EmptyRouteTarget,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvaluationAxis {
    Intent,
    Tooling,
    Complexity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RoutingPrompt(String);

impl RoutingPrompt {
    pub fn new(v: impl Into<String>) -> Result<Self, RouterError> {
        let v = v.into();
        if v.trim().is_empty() {
            return Err(RouterError::EmptyPrompt);
        }
        Ok(Self(v))
    }
}

pub struct RouterMarker;
