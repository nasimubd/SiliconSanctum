#![cfg(target_os = "macos")]

use objc2_metal::{MTLBuffer, MTLCreateSystemDefaultDevice, MTLResource, MTLStorageMode};
use sanctum_core::darwin::{
    direct_io::{ChunkRange, DirectModelFile, SharedMetalBuffer, stream_chunk},
    metal_buffer::{MetalBufferError, NativeSharedBuffer, load_shared_chunks},
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

#[test]
fn workers_preserve_range_order_in_native_buffers() {
    use std::io::Write;
    let device = MTLCreateSystemDefaultDevice().expect("Metal device required");
    let mut fixture = tempfile::NamedTempFile::new().unwrap();
    fixture.write_all(b"firstsecondthird").unwrap();
    let model = DirectModelFile::open(fixture.path()).unwrap();
    let ranges = [
        ChunkRange {
            offset: 11,
            length: 5,
        },
        ChunkRange {
            offset: 0,
            length: 5,
        },
        ChunkRange {
            offset: 5,
            length: 6,
        },
    ];
    let buffers = load_shared_chunks(&device, &model, &ranges, 2, 3 * 16_384).unwrap();
    assert_eq!(buffers[0].as_bytes(), b"third");
    assert_eq!(buffers[1].as_bytes(), b"first");
    assert_eq!(buffers[2].as_bytes(), b"second");
}
