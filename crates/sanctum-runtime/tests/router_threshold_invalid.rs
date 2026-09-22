use sanctum_runtime::router::RouteThresholds;
#[test]
fn rejects_inverted_thresholds() {
    assert!(RouteThresholds::new(0.9, 0.8).is_err());
}
