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

impl RoutingPrompt {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AxisEvaluation {
    pub axis: EvaluationAxis,
    confidence: f64,
}

impl AxisEvaluation {
    pub fn new(axis: EvaluationAxis, confidence: f64) -> Result<Self, RouterError> {
        if !confidence.is_finite() || !(0.0..=1.0).contains(&confidence) {
            return Err(RouterError::ProbabilityOutOfRange);
        }
        Ok(Self { axis, confidence })
    }
}

impl AxisEvaluation {
    #[must_use]
    pub const fn confidence(&self) -> f64 {
        self.confidence
    }
}

pub struct RouterMarker;
