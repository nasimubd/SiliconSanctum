use sanctum_runtime::speculative::AcceptanceMetrics;
#[test]
fn accumulates() {
    let mut m = AcceptanceMetrics::default();
    m.record(4, 3);
    m.record(2, 1);
    assert!((m.rate() - 4.0 / 6.0).abs() < f64::EPSILON);
}
