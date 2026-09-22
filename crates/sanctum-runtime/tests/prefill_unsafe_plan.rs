use sanctum_runtime::prefill::{
    KvQuantization, MemorySnapshot, ModelGeometry, PrefillError, PrefillRequest, PrefillScheduler,
    ScratchBudget, TokenCount, TuningPolicy, TuningThresholds,
};

#[test]
fn rejects_plan_over_scratch_budget() {
    let policy = TuningPolicy::new(
        TuningThresholds::new(1, 2).unwrap(),
        ScratchBudget::new(1).unwrap(),
    )
    .unwrap();
    let request = PrefillRequest::new(
        TokenCount::new(1024).unwrap(),
        ModelGeometry::new(1, 1, 1).unwrap(),
        KvQuantization::Bits4,
    )
    .unwrap();
    let memory = MemorySnapshot {
        total_bytes: 1_000_000,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    assert_eq!(
        PrefillScheduler::new(policy).build_plan(request, memory),
        Err(PrefillError::UnsafeScratch)
    );
}
