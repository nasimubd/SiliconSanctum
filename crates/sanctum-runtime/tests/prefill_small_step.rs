use sanctum_runtime::prefill::{
    KvQuantization, MemorySnapshot, ModelGeometry, PrefillRequest, PrefillScheduler, PrefillStep,
    TokenCount, TuningPolicy,
};

#[test]
fn clamps_small_request_to_small_step() {
    let request = PrefillRequest::new(
        TokenCount::new(200).unwrap(),
        ModelGeometry::new(1, 1, 1).unwrap(),
        KvQuantization::Bits4,
    )
    .unwrap();
    let memory = MemorySnapshot {
        total_bytes: 4 * 1024 * 1024 * 1024,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    assert_eq!(
        PrefillScheduler::new(TuningPolicy::default()).select_step(request, memory),
        PrefillStep::Tokens512
    );
}
