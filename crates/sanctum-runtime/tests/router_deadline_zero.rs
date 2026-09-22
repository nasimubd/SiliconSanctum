use sanctum_runtime::router::FanoutDeadline;
#[test]
fn rejects_zero_deadline() {
    assert!(FanoutDeadline::new(std::time::Duration::ZERO).is_err());
}
