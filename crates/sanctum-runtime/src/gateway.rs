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

impl JaggednessFilter {
    #[must_use]
    pub fn inspect(&self, p: &GatewayPrompt) -> JaggednessReport {
        let mut r = JaggednessReport::clear();
        for e in self
            .arithmetic
            .detect(p)
            .into_iter()
            .chain(self.counting.detect(p))
            .chain(self.temporal.detect(p))
        {
            r.push(e);
        }
        r
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BypassReason {
    Arithmetic,
    Counting,
    Temporal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BypassDecision {
    pub target: EscalationTarget,
    pub reason: BypassReason,
}

#[derive(Debug, Clone, Copy)]
pub struct EscalationPolicy {
    counting: EscalationTarget,
    arithmetic: EscalationTarget,
    temporal: EscalationTarget,
}

impl Default for EscalationPolicy {
    fn default() -> Self {
        Self {
            counting: EscalationTarget::DeterministicInterpreter,
            arithmetic: EscalationTarget::DeterministicInterpreter,
            temporal: EscalationTarget::HeavyReasoningModel,
        }
    }
}

impl EscalationPolicy {
    #[must_use]
    pub const fn for_kind(self, kind: JaggednessKind) -> EscalationTarget {
        match kind {
            JaggednessKind::Counting => self.counting,
            JaggednessKind::MultiHopArithmetic => self.arithmetic,
            JaggednessKind::RelativeTemporal => self.temporal,
        }
    }
}

impl EscalationPolicy {
    #[must_use]
    pub fn decide(self, r: &JaggednessReport) -> Option<BypassDecision> {
        if r.contains(JaggednessKind::MultiHopArithmetic) {
            Some(BypassDecision {
                target: self.arithmetic,
                reason: BypassReason::Arithmetic,
            })
        } else if r.contains(JaggednessKind::Counting) {
            Some(BypassDecision {
                target: self.counting,
                reason: BypassReason::Counting,
            })
        } else if r.contains(JaggednessKind::RelativeTemporal) {
            Some(BypassDecision {
                target: self.temporal,
                reason: BypassReason::Temporal,
            })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GatewayPath {
    SystemOne,
    DeterministicInterpreter,
    HeavyReasoningModel,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GatewayDecision {
    pub path: GatewayPath,
    pub report: JaggednessReport,
}

impl GatewayDecision {
    #[must_use]
    pub fn bypassed(&self) -> bool {
        self.path != GatewayPath::SystemOne
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemOneGateway {
    filter: JaggednessFilter,
    policy: EscalationPolicy,
}

impl SystemOneGateway {
    #[must_use]
    pub const fn new(filter: JaggednessFilter, policy: EscalationPolicy) -> Self {
        Self { filter, policy }
    }
}

impl SystemOneGateway {
    #[must_use]
    pub fn decide(&self, p: &GatewayPrompt) -> GatewayDecision {
        let report = self.filter.inspect(p);
        let path = match self.policy.decide(&report).map(|v| v.target) {
            Some(EscalationTarget::DeterministicInterpreter) => {
                GatewayPath::DeterministicInterpreter
            }
            Some(EscalationTarget::HeavyReasoningModel) => GatewayPath::HeavyReasoningModel,
            None => GatewayPath::SystemOne,
        };
        GatewayDecision { path, report }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GatewayMetrics {
    pub system_one: u64,
    pub deterministic: u64,
    pub heavy: u64,
}

impl GatewayMetrics {
    pub fn record(&mut self, path: GatewayPath) {
        match path {
            GatewayPath::SystemOne => self.system_one = self.system_one.saturating_add(1),
            GatewayPath::DeterministicInterpreter => {
                self.deterministic = self.deterministic.saturating_add(1);
            }
            GatewayPath::HeavyReasoningModel => self.heavy = self.heavy.saturating_add(1),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GatewayEvent {
    Inspected,
    Bypassed(BypassReason),
    Forwarded,
}

#[derive(Debug, Clone)]
pub struct GatewayEventLog {
    capacity: usize,
    entries: std::collections::VecDeque<GatewayEvent>,
}

impl GatewayEventLog {
    pub fn new(capacity: usize) -> Result<Self, GatewayError> {
        if capacity == 0 {
            return Err(GatewayError::InvalidLimit);
        }
        Ok(Self {
            capacity,
            entries: std::collections::VecDeque::with_capacity(capacity),
        })
    }
}

impl GatewayEventLog {
    pub fn push(&mut self, event: GatewayEvent) {
        if self.entries.len() == self.capacity {
            self.entries.pop_front();
        }
        self.entries.push_back(event);
    }
}

impl GatewayEventLog {
    #[must_use]
    pub fn entries(&self) -> &std::collections::VecDeque<GatewayEvent> {
        &self.entries
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DetectionLimits {
    pub max_prompt_bytes: usize,
    pub max_evidence: usize,
}

impl DetectionLimits {
    pub fn new(max_prompt_bytes: usize, max_evidence: usize) -> Result<Self, GatewayError> {
        if max_prompt_bytes == 0 || max_evidence == 0 {
            return Err(GatewayError::InvalidLimit);
        }
        Ok(Self {
            max_prompt_bytes,
            max_evidence,
        })
    }
}

impl Default for DetectionLimits {
    fn default() -> Self {
        Self {
            max_prompt_bytes: 64 * 1024,
            max_evidence: 32,
        }
    }
}

#[must_use]
pub fn contains_digit(text: &str) -> bool {
    text.bytes().any(|v| v.is_ascii_digit())
}

#[must_use]
pub fn arithmetic_operator_count(text: &str) -> usize {
    text.chars()
        .filter(|v| matches!(v, '+' | '-' | '*' | '/' | '%'))
        .count()
}

pub struct GatewayMarker;
