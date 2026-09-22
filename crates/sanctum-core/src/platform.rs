//! Operating-system capability boundaries.

use thiserror::Error;

/// Error returned when an operating-system capability is unavailable.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("{capability} is unsupported on {platform}")]
pub struct UnsupportedPlatform {
    pub capability: &'static str,
    pub platform: &'static str,
}
