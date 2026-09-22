use sanctum_runtime::prefill::{
    KvQuantization, MemorySnapshot, ModelGeometry, PrefillError, PrefillRequest, PrefillScheduler,
    TokenCount, TuningPolicy,
};

#[test]
fn protects_large_context_from_scratch_spike() {
    let request = PrefillRequest::new(
        TokenCount::new(32_768).unwrap(),
        ModelGeometry::new(33, 32, 128).unwrap(),
        KvQuantization::Bits4,
    )
    .unwrap();
    let memory = MemorySnapshot {
        total_bytes: 16 * 1024 * 1024 * 1024,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    assert_eq!(
        PrefillScheduler::new(TuningPolicy::default()).build_plan(request, memory),
        Err(PrefillError::UnsafeScratch)
    );
}
