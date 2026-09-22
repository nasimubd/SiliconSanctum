use sanctum_runtime::arbiter::ArbiterPolicy;
#[test]
fn calculates_policy_thresholds() {
    let value = ArbiterPolicy::new(100, 10, 20).unwrap();
    assert_eq!(value.eviction_threshold(), 80);
    assert_eq!(value.reload_threshold(), 70);
}
