use sanctum_runtime::prefill::MemorySnapshot;

#[test]
fn saturates_exhausted_memory() {
    let memory = MemorySnapshot {
        total_bytes: 1024,
        resident_bytes: 2048,
        reserved_bytes: 512,
    };
    assert_eq!(memory.available_bytes(), 0);
}
