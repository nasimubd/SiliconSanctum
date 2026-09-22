//! Operating-system capability boundaries.

use thiserror::Error;

/// Error returned when an operating-system capability is unavailable.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("{capability} is unsupported on {platform}")]
pub struct UnsupportedPlatform {
    pub capability: &'static str,
    pub platform: &'static str,
}

#[cfg(test)]
mod tests {
    use super::UnsupportedPlatform;

    #[test]
    fn unsupported_platform_display_names_capability_and_platform() {
        let error = UnsupportedPlatform {
            capability: "memory pressure",
            platform: "linux",
        };

        assert_eq!(error.to_string(), "memory pressure is unsupported on linux");
    }
}
