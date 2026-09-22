//! System One gateway guardrails.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayError {
    EmptyPrompt,
    EmptyPattern,
    InvalidLimit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayPrompt(String);

impl GatewayPrompt {
    pub fn new(v: impl Into<String>) -> Result<Self, GatewayError> {
        let v = v.into();
        if v.trim().is_empty() {
            return Err(GatewayError::EmptyPrompt);
        }
        Ok(Self(v))
    }
}

impl GatewayPrompt {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl GatewayPrompt {
    #[must_use]
    pub fn normalized(&self) -> String {
        self.0.to_lowercase()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum JaggednessKind {
    MultiHopArithmetic,
    Counting,
    RelativeTemporal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EscalationTarget {
    DeterministicInterpreter,
    HeavyReasoningModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    pub kind: JaggednessKind,
    pub marker: String,
}

impl Evidence {
    pub fn new(kind: JaggednessKind, marker: impl Into<String>) -> Result<Self, GatewayError> {
        let marker = marker.into();
        if marker.is_empty() {
            return Err(GatewayError::EmptyPattern);
        }
        Ok(Self { kind, marker })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JaggednessReport {
    evidence: Vec<Evidence>,
}

impl JaggednessReport {
    #[must_use]
    pub const fn clear() -> Self {
        Self {
            evidence: Vec::new(),
        }
    }
}

impl JaggednessReport {
    #[must_use]
    pub fn is_jagged(&self) -> bool {
        !self.evidence.is_empty()
    }
}

impl JaggednessReport {
    #[must_use]
    pub fn evidence(&self) -> &[Evidence] {
        &self.evidence
    }
}

impl JaggednessReport {
    #[must_use]
    pub fn contains(&self, kind: JaggednessKind) -> bool {
        self.evidence.iter().any(|item| item.kind == kind)
    }
}

impl JaggednessReport {
    pub fn push(&mut self, evidence: Evidence) {
        if !self.evidence.contains(&evidence) {
            self.evidence.push(evidence);
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternSet {
    patterns: Vec<String>,
}

impl PatternSet {
    pub fn new(patterns: Vec<String>) -> Result<Self, GatewayError> {
        if patterns.is_empty() || patterns.iter().any(String::is_empty) {
            return Err(GatewayError::EmptyPattern);
        }
        Ok(Self { patterns })
    }
}

impl PatternSet {
    pub fn matching<'a>(&'a self, text: &'a str) -> impl Iterator<Item = &'a str> {
        self.patterns
            .iter()
            .map(String::as_str)
            .filter(|pattern| text.contains(pattern))
    }
}

#[derive(Debug, Clone)]
pub struct ArithmeticDetector {
    patterns: PatternSet,
}

impl Default for ArithmeticDetector {
    fn default() -> Self {
        Self {
            patterns: PatternSet::new(vec![
                "calculate".into(),
                "sum".into(),
                "multiply".into(),
                "percent".into(),
                "then".into(),
            ])
            .expect("static patterns"),
        }
    }
}

impl ArithmeticDetector {
    #[must_use]
    pub fn detect(&self, p: &GatewayPrompt) -> Vec<Evidence> {
        let text = p.normalized();
        let hits: Vec<_> = self.patterns.matching(&text).collect();
        if hits.len() >= 2 {
            hits.into_iter()
                .map(|v| Evidence {
                    kind: JaggednessKind::MultiHopArithmetic,
                    marker: v.to_owned(),
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CountingDetector {
    patterns: PatternSet,
}

impl Default for CountingDetector {
    fn default() -> Self {
        Self {
            patterns: PatternSet::new(vec![
                "count".into(),
                "how many".into(),
                "number of".into(),
                "enumerate".into(),
            ])
            .expect("static patterns"),
        }
    }
}

impl CountingDetector {
    #[must_use]
    pub fn detect(&self, p: &GatewayPrompt) -> Vec<Evidence> {
        let text = p.normalized();
        self.patterns
            .matching(&text)
            .map(|v| Evidence {
                kind: JaggednessKind::Counting,
                marker: v.to_owned(),
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct TemporalDetector {
    patterns: PatternSet,
}

impl Default for TemporalDetector {
    fn default() -> Self {
        Self {
            patterns: PatternSet::new(vec![
                "before".into(),
                "after".into(),
                "earlier".into(),
                "later".into(),
                "from now".into(),
                "ago".into(),
            ])
            .expect("static patterns"),
        }
    }
}

impl TemporalDetector {
    #[must_use]
    pub fn detect(&self, p: &GatewayPrompt) -> Vec<Evidence> {
        let text = p.normalized();
        self.patterns
            .matching(&text)
            .map(|v| Evidence {
                kind: JaggednessKind::RelativeTemporal,
                marker: v.to_owned(),
            })
            .collect()
    }
}

#[derive(Debug, Clone, Default)]
pub struct JaggednessFilter {
    arithmetic: ArithmeticDetector,
    counting: CountingDetector,
    temporal: TemporalDetector,
}

pub struct GatewayMarker;
