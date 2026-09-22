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

impl LatencyBudget {
    pub fn new(
        minimum: std::time::Duration,
        maximum: std::time::Duration,
    ) -> Result<Self, DecisionError> {
        if minimum < MIN_DECISION_LATENCY {
            return Err(DecisionError::LatencyBelowMinimum);
        }
        if maximum > MAX_DECISION_LATENCY {
            return Err(DecisionError::LatencyAboveMaximum);
        }
        if minimum > maximum {
            return Err(DecisionError::InvertedLatencyRange);
        }
        Ok(Self { minimum, maximum })
    }
}

impl Default for LatencyBudget {
    fn default() -> Self {
        Self {
            minimum: MIN_DECISION_LATENCY,
            maximum: MAX_DECISION_LATENCY,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DecisionModelId(String);

impl DecisionModelId {
    pub fn new(value: impl Into<String>) -> Result<Self, DecisionError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(DecisionError::EmptyField { field: "model_id" });
        }
        Ok(Self(value))
    }
}

impl DecisionModelId {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub const MAX_DECISION_PARAMETERS: u64 = 200_000_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionModelMetadata {
    pub id: DecisionModelId,
    parameters: u64,
}

impl DecisionModelMetadata {
    pub fn new(id: DecisionModelId, parameters: u64) -> Result<Self, DecisionError> {
        if parameters == 0 {
            return Err(DecisionError::ZeroParameterCount);
        }
        if parameters >= MAX_DECISION_PARAMETERS {
            return Err(DecisionError::ParameterLimitExceeded { parameters });
        }
        Ok(Self { id, parameters })
    }
}

impl DecisionModelMetadata {
    #[must_use]
    pub const fn parameters(&self) -> u64 {
        self.parameters
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InferenceBackend {
    Onnx,
    CoreMl,
    Candle,
}

pub struct DecisionMarker;
