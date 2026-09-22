use sanctum_runtime::router::RouteThresholds;
#[test]
fn provides_default_thresholds() {
    let v = RouteThresholds::default();
    assert!((v.moderate() - 0.60).abs() < f64::EPSILON);
    assert!((v.high() - 0.95).abs() < f64::EPSILON);
}
