use sanctum_runtime::tui::{MemoryBytes, MemoryTelemetry};
#[test]
fn rejects_excess_wired() {
    assert!(MemoryTelemetry::new(MemoryBytes(17), MemoryBytes(1), MemoryBytes(16)).is_err());
}
