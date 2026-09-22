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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteThresholds {
    moderate: f64,
    high: f64,
}

impl Default for RouteThresholds {
    fn default() -> Self {
        Self {
            moderate: 0.60,
            high: 0.95,
        }
    }
}

impl RouteThresholds {
    pub fn new(moderate: f64, high: f64) -> Result<Self, RouterError> {
        if !moderate.is_finite()
            || !high.is_finite()
            || moderate < 0.0
            || high > 1.0
            || moderate >= high
        {
            return Err(RouterError::InvalidThresholds);
        }
        Ok(Self { moderate, high })
    }
}

impl RouteThresholds {
    #[must_use]
    pub const fn moderate(self) -> f64 {
        self.moderate
    }
}

impl RouteThresholds {
    #[must_use]
    pub const fn high(self) -> f64 {
        self.high
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteDestination {
    DeterministicTool,
    SpeculativePipeline,
    HeavyModel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RouteReason {
    HighCertainty,
    ModerateCertainty,
    LowCertainty,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RouteDecision {
    pub destination: RouteDestination,
    pub reason: RouteReason,
    pub confidence: f64,
}

impl RouteDecision {
    #[must_use]
    pub const fn deterministic(c: f64) -> Self {
        Self {
            destination: RouteDestination::DeterministicTool,
            reason: RouteReason::HighCertainty,
            confidence: c,
        }
    }
}

impl RouteDecision {
    #[must_use]
    pub const fn speculative(c: f64) -> Self {
        Self {
            destination: RouteDestination::SpeculativePipeline,
            reason: RouteReason::ModerateCertainty,
            confidence: c,
        }
    }
}

pub struct RouterMarker;
