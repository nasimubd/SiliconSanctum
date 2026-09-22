//! Typed access to Darwin kernel controls.

use thiserror::Error;

pub const IOGPU_WIRED_LIMIT_KEY: &str = "iogpu.wired_limit_mb";

#[derive(Debug, Error)]
pub enum SysctlError {
    #[error("sysctl key contains an interior NUL byte: {0}")]
    InvalidKey(String),
    #[error("sysctl {operation} failed for {key}: {source}")]
    Operation {
        operation: &'static str,
        key: String,
        #[source]
        source: std::io::Error,
    },
    #[error("sysctl {key} returned {actual} bytes; expected {expected}")]
    InvalidWidth {
        key: String,
        expected: usize,
        actual: usize,
    },
}
