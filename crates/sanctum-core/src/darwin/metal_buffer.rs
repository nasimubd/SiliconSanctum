//! Owned page-aligned shared Metal resources.

use std::ptr::NonNull;

use objc2::{rc::Retained, runtime::ProtocolObject};
use objc2_metal::{MTLBuffer, MTLDevice, MTLResourceOptions};
use thiserror::Error;

use super::{
    APPLE_SILICON_PAGE_SIZE,
    direct_io::{AlignedBuffer, DirectIoError, SharedMetalBuffer},
};

use super::direct_io::{ChunkRange, DirectModelFile};

#[derive(Debug, Error)]
pub enum MetalBufferError {
    #[error("shared buffer size exceeds the allocation budget or device limit")]
    Capacity,
    #[error("Metal refused the shared buffer allocation")]
    Unavailable,
    #[error(transparent)]
    Allocation(#[from] DirectIoError),
    #[error("failed to start storage worker: {0}")]
    WorkerStart(#[source] std::io::Error),
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

/// Loads file ranges directly into native shared Metal allocations on workers.
///
/// Includes page padding in the total budget; preserves range order. The caller
/// must subtract existing model, KV, scratch, and OS residency from this budget.
///
/// # Errors
///
/// Rejects invalid ranges, zero worker count, insufficient budget, or worker/I/O failures.
pub fn load_shared_chunks(
    device: &ProtocolObject<dyn MTLDevice>,
    model: &DirectModelFile,
    ranges: &[ChunkRange],
    worker_limit: usize,
    budget_bytes: usize,
) -> Result<Vec<NativeSharedBuffer>, MetalBufferError> {
    if worker_limit == 0 {
        return Err(DirectIoError::InvalidWorkerLimit.into());
    }
    let mut remaining = budget_bytes;
    // Preflight the entire request before any allocation.
    for range in ranges {
        if range.end().is_none_or(|end| i64::try_from(end).is_err()) {
            return Err(DirectIoError::OffsetOverflow {
                offset: range.offset,
            }
            .into());
        }
        let padded = range
            .length
            .max(1)
            .checked_add(APPLE_SILICON_PAGE_SIZE - 1)
            .map(|n| n & !(APPLE_SILICON_PAGE_SIZE - 1))
            .ok_or(MetalBufferError::Capacity)?;
        remaining = remaining
            .checked_sub(padded)
            .ok_or(MetalBufferError::Capacity)?;
    }
    let mut output = ranges
        .iter()
        .map(|range| NativeSharedBuffer::new(device, range.length, budget_bytes))
        .collect::<Result<Vec<_>, _>>()?;
    for (sinks, ranges) in output
        .chunks_mut(worker_limit)
        .zip(ranges.chunks(worker_limit))
    {
        std::thread::scope(|scope| {
            let mut handles = Vec::new();
            let mut first_error = None;
            for (sink, range) in sinks.iter_mut().zip(ranges.iter().copied()) {
                // Only the exclusive CPU slice crosses the thread boundary.
                // The Metal object stays here and outlives every scoped worker.
                let bytes = sink.writable_bytes();
                match std::thread::Builder::new()
                    .name("sanctum-read".into())
                    .spawn_scoped(scope, move || model.read_exact_at(bytes, range.offset))
                {
                    Ok(handle) => handles.push(handle),
                    Err(error) => {
                        first_error = Some(MetalBufferError::WorkerStart(error));
                        break;
                    }
                }
            }
            for handle in handles {
                let result = handle
                    .join()
                    .map_err(|_| DirectIoError::WorkerPanic)
                    .and_then(std::convert::identity);
                if let Err(error) = result {
                    first_error.get_or_insert(error.into());
                }
            }
            first_error.map_or(Ok(()), Err)
        })?;
    }
    Ok(output)
}
