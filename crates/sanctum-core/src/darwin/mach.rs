//! Mach host memory telemetry.

use thiserror::Error;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MachTelemetryError {
    #[error("Mach {operation} failed with kernel status {status}")]
    Kernel {
        operation: &'static str,
        status: i32,
    },
    #[error("memory counter overflowed while converting {counter} pages")]
    CounterOverflow { counter: &'static str },
}

#[cfg(test)]
mod tests {
    use super::MachTelemetryError;

    #[test]
    fn kernel_error_display_names_operation_and_status() {
        let error = MachTelemetryError::Kernel {
            operation: "host_statistics64",
            status: 5,
        };
        assert_eq!(
            error.to_string(),
            "Mach host_statistics64 failed with kernel status 5"
        );
    }
}
