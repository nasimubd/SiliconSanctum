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

pub struct GatewayMarker;
