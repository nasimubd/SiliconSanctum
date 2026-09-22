#![cfg(target_os = "macos")]

use objc2_metal::{MTLBuffer, MTLCreateSystemDefaultDevice, MTLResource, MTLStorageMode};
use sanctum_core::darwin::{
    direct_io::{ChunkRange, DirectModelFile, SharedMetalBuffer, stream_chunk},
    metal_buffer::{MetalBufferError, NativeSharedBuffer},
};

#[test]
fn model_bytes_share_the_native_metal_allocation() {
    use std::io::Write;
    let device = MTLCreateSystemDefaultDevice().expect("Metal device required");
    let mut fixture = tempfile::NamedTempFile::new().unwrap();
    let bytes = vec![0x6d; 16_391];
    fixture.write_all(&bytes).unwrap();
    let model = DirectModelFile::open(fixture.path()).unwrap();
    let mut sink = NativeSharedBuffer::new(&device, bytes.len(), 32_768).unwrap();
    assert_eq!(sink.capacity(), 32_768);
    assert_eq!(
        stream_chunk(
            &model,
            ChunkRange {
                offset: 0,
                length: bytes.len()
            },
            &mut sink
        )
        .unwrap(),
        bytes.len()
    );
    let pointer = sink.writable_bytes().as_ptr();
    // SAFETY: no commands are submitted and the resource borrow stays within owner lifetime.
    let resource = unsafe { sink.resource() };
    assert_eq!(resource.storageMode(), MTLStorageMode::Shared);
    assert_eq!(
        resource.contents().as_ptr().cast_const().cast::<u8>(),
        pointer
    );
    assert_eq!(pointer.addr() % 16_384, 0);
    assert_eq!(sink.as_bytes(), bytes);
}

#[test]
fn metal_capacity_checks_include_page_rounding() {
    let device = MTLCreateSystemDefaultDevice().expect("Metal device required");
    assert!(matches!(
        NativeSharedBuffer::new(&device, 16_385, 16_385),
        Err(MetalBufferError::Capacity)
    ));
    assert!(matches!(
        NativeSharedBuffer::new(&device, usize::MAX, usize::MAX),
        Err(MetalBufferError::Capacity)
    ));
}
