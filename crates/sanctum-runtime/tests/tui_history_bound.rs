use sanctum_runtime::tui::*;
#[test]
fn evicts_oldest_sample() {
    let mut history = MetricHistory::new(1).unwrap();
    let mk = |rate| {
        DashboardSnapshot::new(
            TokenRate::new(rate).unwrap(),
            MemoryTelemetry::new(MemoryBytes(0), MemoryBytes(0), MemoryBytes(1)).unwrap(),
            CpuTelemetry {
                performance: Utilization::new(0.0).unwrap(),
                efficiency: Utilization::new(0.0).unwrap(),
            },
            KvResidency::new(0, 1).unwrap(),
            ActiveProfile::new("idle").unwrap(),
        )
    };
    history.record(&mk(1.0));
    history.record(&mk(2.0));
    assert_eq!(history.token_rates().len(), 1);
    assert!((history.token_rates()[0] - 2.0).abs() < f64::EPSILON);
}
