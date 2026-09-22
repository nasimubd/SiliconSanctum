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

#[cfg(test)]
mod tests {
    use super::SysctlError;

    #[test]
    fn invalid_width_display_includes_key_and_sizes() {
        let error = SysctlError::InvalidWidth {
            key: "example".into(),
            expected: 8,
            actual: 4,
        };

        assert_eq!(
            error.to_string(),
            "sysctl example returned 4 bytes; expected 8"
        );
    }
}
