//! Zero-swap memory arbitration.
#![allow(clippy::missing_errors_doc)]

pub const BYTES_PER_GIB: u64 = 1_073_741_824;
pub const WIRED_LIMIT_BYTES: u64 = 10_400 * 1_048_576;
pub const DEFAULT_HEADROOM_BYTES: u64 = 512 * 1_048_576;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MemorySnapshot {
    pub wired_bytes: u64,
    pub available_bytes: u64,
    pub swap_used_bytes: u64,
}

impl MemorySnapshot {
    #[must_use]
    pub const fn new(wired_bytes: u64, available_bytes: u64, swap_used_bytes: u64) -> Self {
        Self {
            wired_bytes,
            available_bytes,
            swap_used_bytes,
        }
    }

    #[must_use]
    pub const fn wired_bytes(self) -> u64 {
        self.wired_bytes
    }
    #[must_use]
    pub const fn available_bytes(self) -> u64 {
        self.available_bytes
    }
    #[must_use]
    pub const fn swap_used_bytes(self) -> u64 {
        self.swap_used_bytes
    }
}
