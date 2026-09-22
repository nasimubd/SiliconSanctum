use sanctum_runtime::prefill::{MemoryPressure, MemorySnapshot, TuningThresholds};

#[test]
fn derives_memory_pressure_decisions() {
    let thresholds = TuningThresholds::new(100, 200).unwrap();
    let snapshot = |available| MemorySnapshot {
        total_bytes: available,
        resident_bytes: 0,
        reserved_bytes: 0,
    };
    assert_eq!(thresholds.pressure(snapshot(50)), MemoryPressure::Critical);
    assert_eq!(thresholds.pressure(snapshot(150)), MemoryPressure::Warning);
    assert_eq!(thresholds.pressure(snapshot(250)), MemoryPressure::Normal);
}
