//! Owned page-aligned shared Metal resources.

use std::ptr::NonNull;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLBuffer, MTLDevice, MTLResourceOptions};
use thiserror::Error;

use super::{
    APPLE_SILICON_PAGE_SIZE,
    direct_io::{AlignedBuffer, DirectIoError, SharedMetalBuffer},
};

#[derive(Debug, Error)]
pub enum MetalBufferError {
    #[error("shared buffer size exceeds the allocation budget or device limit")]
    Capacity,
    #[error("Metal refused the shared buffer allocation")]
    Unavailable,
    #[error(transparent)]
    Allocation(#[from] DirectIoError),
}

/// Owns a Metal object and its no-copy CPU backing.
///
/// Field order releases the Metal object before freeing the backing allocation.
/// GPU handles are available only through an unsafe synchronization contract.
pub struct NativeSharedBuffer {
    resource: Retained<ProtocolObject<dyn MTLBuffer>>,
    backing: AlignedBuffer,
    logical_length: usize,
}

impl NativeSharedBuffer {
    /// Allocates page-rounded shared storage within the supplied byte budget.
    ///
    /// # Errors
    ///
    /// Rejects overflow, insufficient budget, device limits, or allocation failure.
    pub fn new(
        device: &ProtocolObject<dyn MTLDevice>,
        length: usize,
        budget_bytes: usize,
    ) -> Result<Self, MetalBufferError> {
        let capacity = length
            .max(1)
            .checked_add(APPLE_SILICON_PAGE_SIZE - 1)
            .map(|n| n & !(APPLE_SILICON_PAGE_SIZE - 1))
            .filter(|n| *n <= budget_bytes && *n <= device.maxBufferLength())
            .ok_or(MetalBufferError::Capacity)?;
        let mut backing = AlignedBuffer::new(capacity, APPLE_SILICON_PAGE_SIZE)?;
        let pointer = NonNull::from(&mut backing.as_mut_slice()[0]).cast();
        // SAFETY: both pointer and length are page-aligned. The owned backing
        // outlives the resource; no deallocator is installed because Rust owns it.
        let resource = unsafe {
            device.newBufferWithBytesNoCopy_length_options_deallocator(
                pointer,
                capacity,
                MTLResourceOptions::StorageModeShared,
                None,
            )
        }
        .ok_or(MetalBufferError::Unavailable)?;
        Ok(Self {
            resource,
            backing,
            logical_length: length,
        })
    }

    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.backing.as_slice()[..self.logical_length]
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.backing.len()
    }

    /// Borrows the Metal resource for command encoding.
    ///
    /// # Safety
    ///
    /// All GPU accesses must finish before accessing the CPU slices or dropping
    /// this owner. Retained copies of the resource must not outlive this owner.
    #[must_use]
    pub unsafe fn resource(&self) -> &ProtocolObject<dyn MTLBuffer> {
        &self.resource
    }
}

impl SharedMetalBuffer for NativeSharedBuffer {
    fn writable_bytes(&mut self) -> &mut [u8] {
        &mut self.backing.as_mut_slice()[..self.logical_length]
    }
}
