//! Zero-swap memory arbitration.
#![allow(clippy::missing_errors_doc)]

pub const BYTES_PER_GIB: u64 = 1_073_741_824;
pub const WIRED_LIMIT_BYTES: u64 = 10_400 * 1_048_576;
