use sanctum_runtime::speculative::{TelemetryDto, TelemetrySnapshot};
#[test]
fn converts_snapshot() {
    let dto = TelemetryDto::from(TelemetrySnapshot {
        tokens_per_second: 1.0,
        acceptance_rate: 0.5,
        verify_width: 2,
        context_pressure: 0.3,
        bandwidth_utilization: 0.4,
    });
    assert_eq!(dto.verify_width, 2);
}
