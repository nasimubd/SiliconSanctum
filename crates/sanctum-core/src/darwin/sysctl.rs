//! Typed access to Darwin kernel controls.

use thiserror::Error;

#[cfg(target_os = "macos")]
use std::ffi::CString;

pub const IOGPU_WIRED_LIMIT_KEY: &str = "iogpu.wired_limit_mb";

pub trait SysctlRead {
    fn read(&self, key: &str) -> Result<Vec<u8>, SysctlError>;
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
    use super::{SysctlError, decode_u64};

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
