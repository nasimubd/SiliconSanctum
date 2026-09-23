use sanctum_runtime::tui::{MemoryBytes, MemoryTelemetry};
#[test]
fn preserves_cache() {
    let m = MemoryTelemetry {
        wired: MemoryBytes(4),
        os_cache: MemoryBytes(3),
        total: MemoryBytes(16),
    };
    assert_eq!(m.os_cache.0, 3);
}
