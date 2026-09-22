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
