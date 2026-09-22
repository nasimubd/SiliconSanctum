//! Mach host memory telemetry.

use thiserror::Error;

#[cfg(target_os = "macos")]
unsafe extern "C" {
    fn mach_host_self() -> libc::mach_port_t;
}

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct VmPageCounters {
    pub free: u64,
    pub active: u64,
    pub inactive: u64,
    pub wired: u64,
    pub compressed: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MemoryTelemetry {
    pub page_size: u64,
    pub free_bytes: u64,
    pub active_bytes: u64,
    pub inactive_bytes: u64,
    pub wired_bytes: u64,
    pub compressed_bytes: u64,
}

/// Converts a named page counter into bytes without wrapping.
///
/// # Errors
///
/// Returns [`MachTelemetryError::CounterOverflow`] when multiplication overflows.
pub fn pages_to_bytes(
    counter: &'static str,
    pages: u64,
    page_size: u64,
) -> Result<u64, MachTelemetryError> {
    pages
        .checked_mul(page_size)
        .ok_or(MachTelemetryError::CounterOverflow { counter })
}

pub trait MachHost {
    /// Returns the VM page size in bytes.
    ///
    /// # Errors
    ///
    /// Returns a kernel status when the host query fails.
    fn page_size(&self) -> Result<u64, MachTelemetryError>;

    /// Returns an atomic snapshot of the host VM counters.
    ///
    /// # Errors
    ///
    /// Returns a kernel status when the statistics query fails.
    fn vm_counters(&self) -> Result<VmPageCounters, MachTelemetryError>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct NativeMachHost;

#[cfg(target_os = "macos")]
#[must_use]
pub fn native_page_size() -> u64 {
    // SAFETY: libSystem initializes the read-only Mach page-size global before main.
    u64::try_from(unsafe { mach2::vm_page_size::vm_page_size }).unwrap_or(u64::MAX)
}

#[cfg(target_os = "macos")]
/// Reads VM counters from `host_statistics64`.
///
/// # Errors
///
/// Returns the Mach kernel status when the host statistics call fails.
pub fn native_vm_counters() -> Result<VmPageCounters, MachTelemetryError> {
    let mut statistics = std::mem::MaybeUninit::<libc::vm_statistics64>::zeroed();
    let mut count = libc::HOST_VM_INFO64_COUNT;
    // SAFETY: the output points to a correctly sized zeroed statistics object and
    // `count` is initialized with the ABI-provided element count.
    let status = unsafe {
        libc::host_statistics64(
            mach_host_self(),
            libc::HOST_VM_INFO64,
            statistics.as_mut_ptr().cast(),
            &raw mut count,
        )
    };
    if status != libc::KERN_SUCCESS {
        return Err(MachTelemetryError::Kernel {
            operation: "host_statistics64",
            status,
        });
    }
    // SAFETY: a successful kernel call initialized the complete output structure.
    let statistics = unsafe { statistics.assume_init() };
    Ok(VmPageCounters {
        free: u64::from(statistics.free_count),
        active: u64::from(statistics.active_count),
        inactive: u64::from(statistics.inactive_count),
        wired: u64::from(statistics.wire_count),
        compressed: u64::from(statistics.compressor_page_count),
    })
}

#[cfg(target_os = "macos")]
impl MachHost for NativeMachHost {
    fn page_size(&self) -> Result<u64, MachTelemetryError> {
        Ok(native_page_size())
    }

    fn vm_counters(&self) -> Result<VmPageCounters, MachTelemetryError> {
        native_vm_counters()
    }
}

#[cfg(test)]
mod tests {
    use super::MachTelemetryError;

    #[test]
    fn kernel_error_display_names_operation_and_status() {
        let error = MachTelemetryError::Kernel {
            operation: "host_statistics64",
            status: 5,
        };
        assert_eq!(
            error.to_string(),
            "Mach host_statistics64 failed with kernel status 5"
        );
    }
}
