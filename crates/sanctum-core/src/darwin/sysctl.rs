//! Typed access to Darwin kernel controls.

use thiserror::Error;

#[cfg(target_os = "macos")]
use std::ffi::CString;

pub const IOGPU_WIRED_LIMIT_KEY: &str = "iogpu.wired_limit_mb";

pub trait SysctlRead {
    fn read(&self, key: &str) -> Result<Vec<u8>, SysctlError>;
}

pub trait SysctlWrite {
    fn write(&self, key: &str, value: &[u8]) -> Result<(), SysctlError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NativeSysctl;

#[cfg(target_os = "macos")]
fn key_c_string(key: &str) -> Result<CString, SysctlError> {
    CString::new(key).map_err(|_| SysctlError::InvalidKey(key.into()))
}

#[cfg(target_os = "macos")]
fn query_size(key: &str, native_key: &CString) -> Result<usize, SysctlError> {
    let mut size = 0_usize;
    // SAFETY: `native_key` is NUL terminated and `size` points to writable memory.
    let result = unsafe {
        libc::sysctlbyname(
            native_key.as_ptr(),
            std::ptr::null_mut(),
            &mut size,
            std::ptr::null_mut(),
            0,
        )
    };
    if result == -1 {
        return Err(SysctlError::Operation {
            operation: "size query",
            key: key.into(),
            source: std::io::Error::last_os_error(),
        });
    }
    Ok(size)
}

#[cfg(target_os = "macos")]
impl SysctlRead for NativeSysctl {
    fn read(&self, key: &str) -> Result<Vec<u8>, SysctlError> {
        let native_key = key_c_string(key)?;
        let mut size = query_size(key, &native_key)?;
        let mut value = vec![0_u8; size];
        // SAFETY: both pointers remain valid for the duration of the call and
        // `size` describes the writable allocation.
        let result = unsafe {
            libc::sysctlbyname(
                native_key.as_ptr(),
                value.as_mut_ptr().cast(),
                &mut size,
                std::ptr::null_mut(),
                0,
            )
        };
        if result == -1 {
            return Err(SysctlError::Operation {
                operation: "read",
                key: key.into(),
                source: std::io::Error::last_os_error(),
            });
        }
        value.truncate(size);
        Ok(value)
    }
}

#[cfg(target_os = "macos")]
impl SysctlWrite for NativeSysctl {
    fn write(&self, key: &str, value: &[u8]) -> Result<(), SysctlError> {
        let native_key = key_c_string(key)?;
        // SAFETY: the key is NUL terminated and `value` is readable for its length.
        let result = unsafe {
            libc::sysctlbyname(
                native_key.as_ptr(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                value.as_ptr().cast_mut().cast(),
                value.len(),
            )
        };
        if result == -1 {
            return Err(SysctlError::Operation {
                operation: "write",
                key: key.into(),
                source: std::io::Error::last_os_error(),
            });
        }
        Ok(())
    }
}

fn decode_u64(key: &str, bytes: &[u8]) -> Result<u64, SysctlError> {
    let value: [u8; size_of::<u64>()] =
        bytes.try_into().map_err(|_| SysctlError::InvalidWidth {
            key: key.into(),
            expected: size_of::<u64>(),
            actual: bytes.len(),
        })?;
    Ok(u64::from_ne_bytes(value))
}

pub fn wired_limit_mb(backend: &impl SysctlRead) -> Result<u64, SysctlError> {
    decode_u64(IOGPU_WIRED_LIMIT_KEY, &backend.read(IOGPU_WIRED_LIMIT_KEY)?)
}

pub fn set_wired_limit_mb(
    backend: &impl SysctlWrite,
    limit_mb: u64,
) -> Result<(), SysctlError> {
    backend.write(IOGPU_WIRED_LIMIT_KEY, &limit_mb.to_ne_bytes())
}

pub struct WiredLimitGuard<'a, B: SysctlWrite> {
    backend: &'a B,
    previous_mb: u64,
    armed: bool,
}

impl<B: SysctlWrite> std::fmt::Debug for WiredLimitGuard<'_, B> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WiredLimitGuard")
            .field("previous_mb", &self.previous_mb)
            .field("armed", &self.armed)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Error)]
pub enum SysctlError {
    #[error("sysctl key contains an interior NUL byte: {0}")]
    InvalidKey(String),
    #[error("sysctl {operation} failed for {key}: {source}")]
    Operation {
        operation: &'static str,
        key: String,
        #[source]
        source: std::io::Error,
    },
    #[error("sysctl {key} returned {actual} bytes; expected {expected}")]
    InvalidWidth {
        key: String,
        expected: usize,
        actual: usize,
    },
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::{
        IOGPU_WIRED_LIMIT_KEY, SysctlError, SysctlRead, SysctlWrite, decode_u64,
        set_wired_limit_mb, wired_limit_mb,
    };

    struct FakeRead {
        keys: RefCell<Vec<String>>,
        value: u64,
    }

    impl SysctlRead for FakeRead {
        fn read(&self, key: &str) -> Result<Vec<u8>, SysctlError> {
            self.keys.borrow_mut().push(key.into());
            Ok(self.value.to_ne_bytes().to_vec())
        }
    }

    #[derive(Default)]
    struct FakeWrite(RefCell<Vec<(String, Vec<u8>)>>);

    impl SysctlWrite for FakeWrite {
        fn write(&self, key: &str, value: &[u8]) -> Result<(), SysctlError> {
            self.0.borrow_mut().push((key.into(), value.into()));
            Ok(())
        }
    }

    #[test]
    fn unsigned_values_use_native_byte_order() {
        assert_eq!(decode_u64("example", &42_u64.to_ne_bytes()).unwrap(), 42);
    }

    #[test]
    fn unsigned_values_reject_invalid_width() {
        let error = decode_u64("example", &[0; 4]).unwrap_err();
        assert!(matches!(
            error,
            SysctlError::InvalidWidth {
                expected: 8,
                actual: 4,
                ..
            }
        ));
    }

    #[test]
    fn wired_limit_reader_selects_iogpu_key() {
        let backend = FakeRead {
            keys: RefCell::default(),
            value: 10_240,
        };

        assert_eq!(wired_limit_mb(&backend).unwrap(), 10_240);
        assert_eq!(backend.keys.into_inner(), [IOGPU_WIRED_LIMIT_KEY]);
    }

    #[test]
    fn wired_limit_writer_uses_native_payload() {
        let backend = FakeWrite::default();

        set_wired_limit_mb(&backend, 10_400).unwrap();

        assert_eq!(
            backend.0.into_inner(),
            [(IOGPU_WIRED_LIMIT_KEY.into(), 10_400_u64.to_ne_bytes().into())]
        );
    }

    #[test]
    fn invalid_width_display_includes_key_and_sizes() {
        let error = SysctlError::InvalidWidth {
            key: "example".into(),
            expected: 8,
            actual: 4,
        };

        assert_eq!(
            error.to_string(),
            "sysctl example returned 4 bytes; expected 8"
        );
    }
}
