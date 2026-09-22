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

pub struct GatewayMarker;
