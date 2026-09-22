use sanctum_runtime::prefill::TuningPolicy;

#[test]
fn provides_safe_default_policy() {
    let policy = TuningPolicy::default();
    assert!(policy.thresholds.constrained_bytes < policy.thresholds.comfortable_bytes);
    assert!(policy.scratch_budget.bytes() > 0);
}
