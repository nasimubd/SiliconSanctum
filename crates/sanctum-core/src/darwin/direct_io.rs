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
    #[error("shared Metal buffer has {available} bytes; chunk requires {required}")]
    SharedBufferTooSmall { required: usize, available: usize },
    #[error("shared Metal buffer pointer is not aligned to {required} bytes")]
    SharedBufferMisaligned { required: usize },
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

    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        // SAFETY: the allocation remains live and initialized for `length` bytes.
        unsafe { std::slice::from_raw_parts(self.pointer.as_ptr(), self.length) }
    }

    #[must_use]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        // SAFETY: the exclusive borrow guarantees unique access to the allocation.
        unsafe { std::slice::from_raw_parts_mut(self.pointer.as_ptr(), self.length) }
    }
}

impl Drop for AlignedBuffer {
    fn drop(&mut self) {
        // SAFETY: pointer was returned by posix_memalign and has not been freed.
        unsafe { libc::free(self.pointer.as_ptr().cast()) };
    }
}

#[derive(Debug)]
pub struct DirectModelFile {
    file: std::fs::File,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChunkRange {
    pub offset: u64,
    pub length: usize,
}

pub trait SharedMetalBuffer {
    fn writable_bytes(&mut self) -> &mut [u8];
}

impl ChunkRange {
    #[must_use]
    pub fn end(self) -> Option<u64> {
        self.offset.checked_add(u64::try_from(self.length).ok()?)
    }
}

impl DirectModelFile {
    /// Opens a model file read-only.
    ///
    /// # Errors
    ///
    /// Returns an error when the path cannot be opened.
    pub fn open(path: impl Into<PathBuf>) -> Result<Self, DirectIoError> {
        let path = path.into();
        let file = std::fs::File::open(&path).map_err(|source| DirectIoError::Open {
            path: path.clone(),
            source,
        })?;
        Ok(Self { file })
    }

    #[must_use]
    pub fn raw_fd(&self) -> std::os::fd::RawFd {
        use std::os::fd::AsRawFd;
        self.file.as_raw_fd()
    }

    /// Requests uncached I/O for the model descriptor.
    ///
    /// # Errors
    ///
    /// Returns an error when `fcntl(F_NOCACHE)` fails.
    #[cfg(target_os = "macos")]
    pub fn enable_no_cache(&self) -> Result<(), DirectIoError> {
        // SAFETY: the descriptor is owned and live; F_NOCACHE accepts an integer flag.
        let result = unsafe { libc::fcntl(self.raw_fd(), libc::F_NOCACHE, 1) };
        if result == -1 {
            Err(DirectIoError::NoCache(std::io::Error::last_os_error()))
        } else {
            Ok(())
        }
    }

    /// Reads bytes at an absolute model-file offset without changing file position.
    ///
    /// # Errors
    ///
    /// Returns an error when the offset cannot be represented or `pread` fails.
    pub fn read_at(&self, buffer: &mut [u8], offset: u64) -> Result<usize, DirectIoError> {
        let native_offset =
            libc::off_t::try_from(offset).map_err(|_| DirectIoError::OffsetOverflow { offset })?;
        loop {
            // SAFETY: the descriptor is live and buffer is writable for its full length.
            let count = unsafe {
                libc::pread(
                    self.raw_fd(),
                    buffer.as_mut_ptr().cast(),
                    buffer.len(),
                    native_offset,
                )
            };
            if count >= 0 {
                return usize::try_from(count).map_err(|_| DirectIoError::Read {
                    offset,
                    source: std::io::Error::other("pread returned an invalid byte count"),
                });
            }
            let source = std::io::Error::last_os_error();
            if source.kind() != std::io::ErrorKind::Interrupted {
                return Err(DirectIoError::Read { offset, source });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{AlignedBuffer, DirectIoError, DirectModelFile, validate_layout};

    fn model_fixture(bytes: &[u8]) -> tempfile::NamedTempFile {
        use std::io::Write;
        let mut fixture = tempfile::NamedTempFile::new().unwrap();
        fixture.write_all(bytes).unwrap();
        fixture
    }

    #[test]
    fn positional_read_starts_at_requested_offset() {
        let fixture = model_fixture(b"abcdefgh");
        let model = DirectModelFile::open(fixture.path()).unwrap();
        let mut buffer = [0; 3];
        assert_eq!(model.read_at(&mut buffer, 2).unwrap(), 3);
        assert_eq!(&buffer, b"cde");
    }

    #[test]
    fn positional_read_preserves_short_count() {
        let fixture = model_fixture(b"abc");
        let model = DirectModelFile::open(fixture.path()).unwrap();
        let mut buffer = [0; 8];
        assert_eq!(model.read_at(&mut buffer, 0).unwrap(), 3);
        assert_eq!(&buffer[..3], b"abc");
    }

    #[test]
    fn positional_read_returns_zero_at_eof() {
        let fixture = model_fixture(b"abc");
        let model = DirectModelFile::open(fixture.path()).unwrap();
        let mut buffer = [0; 4];
        assert_eq!(model.read_at(&mut buffer, 3).unwrap(), 0);
    }

    #[test]
    fn chunk_end_rejects_offset_overflow() {
        let range = super::ChunkRange {
            offset: u64::MAX,
            length: 1,
        };
        assert_eq!(range.end(), None);
    }

    #[test]
    fn allocation_honors_sixteen_kibibyte_alignment() {
        let buffer = AlignedBuffer::new(16_384, 16_384).unwrap();
        assert_eq!(buffer.as_slice().as_ptr().addr() % 16_384, 0);
    }

    #[test]
    fn zero_length_allocation_exposes_empty_slice() {
        let buffer = AlignedBuffer::new(0, 16_384).unwrap();
        assert!(buffer.is_empty());
        assert!(buffer.as_slice().is_empty());
    }

    #[test]
    fn mutable_slice_updates_aligned_storage() {
        let mut buffer = AlignedBuffer::new(4, 16_384).unwrap();
        buffer.as_mut_slice().copy_from_slice(&[1, 2, 3, 4]);
        assert_eq!(buffer.as_slice(), [1, 2, 3, 4]);
    }

    #[test]
    fn missing_model_reports_open_error() {
        assert!(matches!(
            super::DirectModelFile::open("/definitely/missing/model.gguf"),
            Err(DirectIoError::Open { .. })
        ));
    }

    #[test]
    fn rejects_non_power_of_two_alignment() {
        assert!(matches!(
            validate_layout(4096, 12),
            Err(DirectIoError::InvalidLayout { alignment: 12, .. })
        ));
    }
}
