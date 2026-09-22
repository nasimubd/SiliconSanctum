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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TensorShape(Vec<usize>);

impl TensorShape {
    pub fn new(dimensions: Vec<usize>) -> Result<Self, DecisionError> {
        if dimensions.is_empty() || dimensions.contains(&0) {
            return Err(DecisionError::ZeroDimension);
        }
        dimensions
            .iter()
            .try_fold(1usize, |size, value| size.checked_mul(*value))
            .ok_or(DecisionError::TensorSizeOverflow)?;
        Ok(Self(dimensions))
    }
}

impl TensorShape {
    pub fn elements(&self) -> Result<usize, DecisionError> {
        self.0
            .iter()
            .try_fold(1usize, |size, value| size.checked_mul(*value))
            .ok_or(DecisionError::TensorSizeOverflow)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceInput(Vec<f32>);

impl InferenceInput {
    pub fn new(values: Vec<f32>) -> Result<Self, DecisionError> {
        if values.is_empty() {
            return Err(DecisionError::EmptyInput);
        }
        if !values.iter().all(|value| value.is_finite()) {
            return Err(DecisionError::NonFiniteInput);
        }
        Ok(Self(values))
    }
}

impl InferenceInput {
    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InferenceLogits(Vec<f32>);

impl InferenceLogits {
    pub fn new(values: Vec<f32>) -> Result<Self, DecisionError> {
        if values.is_empty() {
            return Err(DecisionError::EmptyLogits);
        }
        if !values.iter().all(|value| value.is_finite()) {
            return Err(DecisionError::NonFiniteLogit);
        }
        Ok(Self(values))
    }
}

impl InferenceLogits {
    #[must_use]
    pub fn values(&self) -> &[f32] {
        &self.0
    }
}

pub trait DecisionExecutor {
    fn execute(&mut self, input: &InferenceInput) -> Result<InferenceLogits, DecisionError>;
}

#[derive(Debug, Clone)]
pub struct FixtureExecutor {
    logits: InferenceLogits,
    latency: std::time::Duration,
    invocations: u64,
}

impl FixtureExecutor {
    #[must_use]
    pub const fn new(logits: InferenceLogits, latency: std::time::Duration) -> Self {
        Self {
            logits,
            latency,
            invocations: 0,
        }
    }
}

impl FixtureExecutor {
    #[must_use]
    pub const fn latency(&self) -> std::time::Duration {
        self.latency
    }
}

impl FixtureExecutor {
    #[must_use]
    pub const fn invocations(&self) -> u64 {
        self.invocations
    }
}

impl DecisionExecutor for FixtureExecutor {
    fn execute(&mut self, _input: &InferenceInput) -> Result<InferenceLogits, DecisionError> {
        self.invocations = self.invocations.saturating_add(1);
        Ok(self.logits.clone())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LatencyClass {
    BelowBudget,
    WithinBudget,
    AboveBudget,
}

#[must_use]
pub fn classify_latency(elapsed: std::time::Duration, budget: LatencyBudget) -> LatencyClass {
    if elapsed < budget.minimum {
        LatencyClass::BelowBudget
    } else if elapsed > budget.maximum {
        LatencyClass::AboveBudget
    } else {
        LatencyClass::WithinBudget
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExecutionMeasurement {
    pub logits: InferenceLogits,
    pub elapsed: std::time::Duration,
    pub class: LatencyClass,
}

pub fn execute_measured(
    executor: &mut impl DecisionExecutor,
    input: &InferenceInput,
    budget: LatencyBudget,
) -> Result<ExecutionMeasurement, DecisionError> {
    let started = std::time::Instant::now();
    let logits = executor.execute(input)?;
    let elapsed = started.elapsed();
    Ok(ExecutionMeasurement {
        logits,
        elapsed,
        class: classify_latency(elapsed, budget),
    })
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Noul(f64);

impl Noul {
    pub fn new(probability: f64) -> Result<Self, DecisionError> {
        if !probability.is_finite() || !(0.0..=1.0).contains(&probability) {
            return Err(DecisionError::ProbabilityOutOfRange);
        }
        Ok(Self(probability))
    }
}

impl Noul {
    #[must_use]
    pub const fn probability(self) -> f64 {
        self.0
    }
}

impl Noul {
    #[must_use]
    pub fn is_positive(self) -> bool {
        self.0 >= 0.5
    }
}

impl Noul {
    #[must_use]
    pub fn is_negative(self) -> bool {
        !self.is_positive()
    }
}

pub struct DecisionMarker;
