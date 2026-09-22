use sanctum_runtime::router::HeavyModelTarget;
#[test]
fn rejects_empty_heavy_target() {
    assert!(HeavyModelTarget::new("").is_err());
}
