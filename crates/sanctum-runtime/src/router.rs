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

impl RouteDecision {
    #[must_use]
    pub const fn heavy(c: f64) -> Self {
        Self {
            destination: RouteDestination::HeavyModel,
            reason: RouteReason::LowCertainty,
            confidence: c,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RouterPolicy {
    thresholds: RouteThresholds,
}

impl RouterPolicy {
    #[must_use]
    pub const fn new(thresholds: RouteThresholds) -> Self {
        Self { thresholds }
    }
}

impl Default for RouterPolicy {
    fn default() -> Self {
        Self::new(RouteThresholds::default())
    }
}

impl RouterPolicy {
    #[must_use]
    pub fn select(self, c: f64) -> RouteDecision {
        if c > self.thresholds.high() {
            RouteDecision::deterministic(c)
        } else if c >= self.thresholds.moderate() {
            RouteDecision::speculative(c)
        } else {
            RouteDecision::heavy(c)
        }
    }
}

impl RouterPolicy {
    #[must_use]
    pub const fn thresholds(self) -> RouteThresholds {
        self.thresholds
    }
}

pub trait AxisEvaluator {
    fn evaluate<'a>(
        &'a self,
        axis: EvaluationAxis,
        prompt: &'a RoutingPrompt,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<AxisEvaluation, RouterError>> + Send + 'a>,
    >;
}

#[derive(Debug, Clone)]
pub struct FixedAxisEvaluator {
    confidence: f64,
    delay: std::time::Duration,
}

impl FixedAxisEvaluator {
    pub fn new(confidence: f64, delay: std::time::Duration) -> Result<Self, RouterError> {
        AxisEvaluation::new(EvaluationAxis::Intent, confidence)?;
        Ok(Self { confidence, delay })
    }
}

impl AxisEvaluator for FixedAxisEvaluator {
    fn evaluate<'a>(
        &'a self,
        axis: EvaluationAxis,
        _: &'a RoutingPrompt,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<AxisEvaluation, RouterError>> + Send + 'a>,
    > {
        Box::pin(async move {
            tokio::time::sleep(self.delay).await;
            AxisEvaluation::new(axis, self.confidence)
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FanoutResult {
    pub intent: AxisEvaluation,
    pub tooling: AxisEvaluation,
    pub complexity: AxisEvaluation,
}

impl FanoutResult {
    #[must_use]
    pub fn evaluations(&self) -> [&AxisEvaluation; 3] {
        [&self.intent, &self.tooling, &self.complexity]
    }
}

impl FanoutResult {
    #[must_use]
    pub fn minimum_confidence(&self) -> f64 {
        self.evaluations()
            .into_iter()
            .map(AxisEvaluation::confidence)
            .fold(1.0, f64::min)
    }
}

impl FanoutResult {
    #[must_use]
    pub fn average_confidence(&self) -> f64 {
        self.evaluations()
            .into_iter()
            .map(AxisEvaluation::confidence)
            .sum::<f64>()
            / 3.0
    }
}

impl FanoutResult {
    #[must_use]
    pub fn route(&self, p: RouterPolicy) -> RouteDecision {
        p.select(self.minimum_confidence())
    }
}

pub struct FanoutRouter<I, T, C> {
    intent: I,
    tooling: T,
    complexity: C,
    policy: RouterPolicy,
}

impl<I, T, C> FanoutRouter<I, T, C> {
    #[must_use]
    pub const fn new(intent: I, tooling: T, complexity: C, policy: RouterPolicy) -> Self {
        Self {
            intent,
            tooling,
            complexity,
            policy,
        }
    }
}

impl<I: AxisEvaluator, T: AxisEvaluator, C: AxisEvaluator> FanoutRouter<I, T, C> {
    pub async fn evaluate(&self, p: &RoutingPrompt) -> Result<FanoutResult, RouterError> {
        let (i, t, c) = tokio::join!(
            self.intent.evaluate(EvaluationAxis::Intent, p),
            self.tooling.evaluate(EvaluationAxis::Tooling, p),
            self.complexity.evaluate(EvaluationAxis::Complexity, p)
        );
        Ok(FanoutResult {
            intent: i?,
            tooling: t?,
            complexity: c?,
        })
    }
}

impl<I: AxisEvaluator, T: AxisEvaluator, C: AxisEvaluator> FanoutRouter<I, T, C> {
    pub async fn route(&self, p: &RoutingPrompt) -> Result<RouteDecision, RouterError> {
        Ok(self.evaluate(p).await?.route(self.policy))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelPair {
    draft: String,
    target: String,
}

impl ModelPair {
    pub fn new(d: impl Into<String>, t: impl Into<String>) -> Result<Self, RouterError> {
        let d = d.into();
        let t = t.into();
        if d.trim().is_empty() || t.trim().is_empty() {
            return Err(RouterError::EmptyRouteTarget);
        }
        Ok(Self {
            draft: d,
            target: t,
        })
    }
}

impl ModelPair {
    #[must_use]
    pub fn draft(&self) -> &str {
        &self.draft
    }
}

impl ModelPair {
    #[must_use]
    pub fn target(&self) -> &str {
        &self.target
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeterministicTarget(String);

impl DeterministicTarget {
    pub fn new(v: impl Into<String>) -> Result<Self, RouterError> {
        let v = v.into();
        if v.trim().is_empty() {
            return Err(RouterError::EmptyRouteTarget);
        }
        Ok(Self(v))
    }
}

pub struct RouterMarker;
