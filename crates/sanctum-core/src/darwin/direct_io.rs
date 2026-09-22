//! Page-aligned uncached model I/O.

use std::path::PathBuf;
use std::ptr::NonNull;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectIoError {
    #[error("invalid buffer layout: size={size}, alignment={alignment}")]
    InvalidLayout { size: usize, alignment: usize },
    #[error("posix_memalign failed with errno {code}")]
    Allocation { code: i32 },
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

#[derive(Debug)]
pub struct AlignedBuffer {
    pointer: NonNull<u8>,
    length: usize,
    alignment: usize,
}

impl AlignedBuffer {
    /// Allocates a zeroed buffer with the requested alignment.
    ///
    /// # Errors
    ///
    /// Returns an error for an invalid layout or allocator failure.
    pub fn new(length: usize, alignment: usize) -> Result<Self, DirectIoError> {
        validate_layout(length, alignment)?;
        let mut pointer = std::ptr::null_mut();
        // SAFETY: the layout was validated and pointer addresses writable storage.
        let code = unsafe { libc::posix_memalign(&raw mut pointer, alignment, length.max(1)) };
        if code != 0 {
            return Err(DirectIoError::Allocation { code });
        }
        let pointer =
            NonNull::new(pointer.cast()).ok_or(DirectIoError::Allocation { code: libc::ENOMEM })?;
        // SAFETY: the allocation is valid for at least `length` bytes.
        unsafe { std::ptr::write_bytes(pointer.as_ptr(), 0, length) };
        Ok(Self {
            pointer,
            length,
            alignment,
        })
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.length
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.length == 0
    }

    #[must_use]
    pub const fn alignment(&self) -> usize {
        self.alignment
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        // SAFETY: pointer was returned by posix_memalign and has not been freed.
        unsafe { libc::free(self.pointer.as_ptr().cast()) };
    }
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
