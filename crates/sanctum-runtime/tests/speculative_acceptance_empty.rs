use sanctum_runtime::speculative::AcceptanceMetrics;
#[test]
fn empty_rate_is_zero() {
    assert!(AcceptanceMetrics::default().rate().abs() < f64::EPSILON);
}
