use sanctum_runtime::tui::{CpuTelemetry, Utilization};
#[test]
fn preserves_pcores() {
    let c = CpuTelemetry {
        performance: Utilization::new(0.8).unwrap(),
        efficiency: Utilization::new(0.2).unwrap(),
    };
    assert!((c.performance.get() - 0.8).abs() < f32::EPSILON);
}
