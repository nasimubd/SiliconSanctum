use sanctum_runtime::prefill::{
    KvQuantization, MemorySnapshot, ModelGeometry, PrefillError, PrefillRequest, PrefillScheduler,
    TokenCount, TuningPolicy,
};

#[test]
fn rejects_prefill_under_critical_pressure() {
    let request = PrefillRequest::new(
        TokenCount::new(1024).unwrap(),
        ModelGeometry::new(1, 1, 1).unwrap(),
        KvQuantization::Bits4,
    )
    .unwrap();
    let memory = MemorySnapshot {
        total_bytes: 1024,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    assert_eq!(
        PrefillScheduler::new(TuningPolicy::default()).build_plan(request, memory),
        Err(PrefillError::CriticalPressure)
    );
}
