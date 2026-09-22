use sanctum_runtime::speculative::TelemetrySnapshot;
#[test]
fn preserves_fields() {
    let s = TelemetrySnapshot {
        tokens_per_second: 10.0,
        acceptance_rate: 0.75,
        verify_width: 4,
        context_pressure: 0.5,
        bandwidth_utilization: 0.8,
    };
    assert_eq!(s.verify_width, 4);
    assert!((s.acceptance_rate - 0.75).abs() < f64::EPSILON);
}
