//! Darwin-specific kernel and storage facilities.

/// Virtual-memory page alignment used by Apple Silicon hosts.
pub const APPLE_SILICON_PAGE_SIZE: usize = 16_384;
