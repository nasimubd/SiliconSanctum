//! Bounded System One decision runtime.
#![allow(clippy::missing_errors_doc)]

pub const MIN_DECISION_LATENCY: std::time::Duration = std::time::Duration::from_millis(70);

pub const MAX_DECISION_LATENCY: std::time::Duration = std::time::Duration::from_millis(500);

pub struct DecisionMarker;
