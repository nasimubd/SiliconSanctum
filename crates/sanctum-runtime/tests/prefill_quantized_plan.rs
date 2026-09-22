use sanctum_runtime::prefill::{
    KvQuantization, MemorySnapshot, ModelGeometry, PrefillRequest, PrefillScheduler, PrefillStep,
    TokenCount, TuningPolicy,
};

#[test]
fn plans_quantized_kv_initialization() {
    let request = PrefillRequest::new(
        TokenCount::new(2048).unwrap(),
        ModelGeometry::new(1, 1, 1).unwrap(),
        KvQuantization::Bits4,
    )
    .unwrap();
    let memory = MemorySnapshot {
        total_bytes: 8 * 1024 * 1024 * 1024,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    let plan = PrefillScheduler::new(TuningPolicy::default())
        .build_plan(request, memory)
        .unwrap();
    assert_eq!(plan.step, PrefillStep::Tokens1024);
    assert_eq!(plan.planned_tokens(), 2048);
    assert_eq!(plan.kv_bytes, 2048);
}
