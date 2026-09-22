//! Page-aligned uncached model I/O.

use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectIoError {
    #[error("invalid buffer layout: size={size}, alignment={alignment}")]
    InvalidLayout { size: usize, alignment: usize },
    #[error("failed to open model file {path}: {source}")]
    Open {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to configure F_NOCACHE: {0}")]
    NoCache(#[source] std::io::Error),
    #[error("positional read failed at offset {offset}: {source}")]
    Read {
        offset: u64,
        #[source]
        source: std::io::Error,
    },
    #[error("model offset {offset} does not fit off_t")]
    OffsetOverflow { offset: u64 },
}

/// Validates the alignment accepted by `posix_memalign`.
///
/// # Errors
///
/// Returns an error unless alignment is a pointer-sized power of two.
pub fn validate_layout(size: usize, alignment: usize) -> Result<(), DirectIoError> {
    if alignment < size_of::<*const ()>()
        || !alignment.is_power_of_two()
        || alignment % size_of::<*const ()>() != 0
    {
        return Err(DirectIoError::InvalidLayout { size, alignment });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{DirectIoError, validate_layout};

    #[test]
    fn rejects_non_power_of_two_alignment() {
        assert!(matches!(
            validate_layout(4096, 12),
            Err(DirectIoError::InvalidLayout { alignment: 12, .. })
        ));
    }
}
