use sanctum_runtime::arbiter::ArbiterPolicy;
#[test]
fn rejects_zero_wired_limit() {
    assert!(ArbiterPolicy::new(0, 1, 1).is_err());
}
