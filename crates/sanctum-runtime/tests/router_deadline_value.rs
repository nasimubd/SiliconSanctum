use sanctum_runtime::router::FanoutDeadline;
#[test]
fn preserves_deadline() {
    let d = std::time::Duration::from_millis(500);
    assert_eq!(FanoutDeadline::new(d).unwrap().duration(), d);
}
