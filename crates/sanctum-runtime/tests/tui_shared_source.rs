use sanctum_runtime::tui::*;
#[test]
fn reads_latest_publication() {
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
    let mut source = SharedTelemetrySource::new(mk(1.0));
    let publisher = source.clone();
    publisher.publish(mk(2.0)).unwrap();
    assert!((source.snapshot().unwrap().token_rate.get() - 2.0).abs() < f64::EPSILON);
}
