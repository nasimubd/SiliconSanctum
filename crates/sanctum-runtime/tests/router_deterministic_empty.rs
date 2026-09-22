use sanctum_runtime::router::DeterministicTarget;
#[test]
fn rejects_empty_deterministic_target() {
    assert!(DeterministicTarget::new("").is_err());
}
