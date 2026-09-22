//! Darwin-specific kernel and storage facilities.

/// Virtual-memory page alignment used by Apple Silicon hosts.
pub const APPLE_SILICON_PAGE_SIZE: usize = 16_384;

pub mod direct_io;
pub mod mach;
pub mod pressure;
pub mod qos;
pub mod sysctl;

#[cfg(test)]
mod tests {
    use super::APPLE_SILICON_PAGE_SIZE;

    #[test]
    fn apple_silicon_page_size_is_sixteen_kibibytes() {
        assert_eq!(APPLE_SILICON_PAGE_SIZE, 16 * 1024);
    }
}
