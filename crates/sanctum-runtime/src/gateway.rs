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

pub struct GatewayMarker;
