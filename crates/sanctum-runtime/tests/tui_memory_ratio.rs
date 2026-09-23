use sanctum_runtime::tui::MemoryBytes;
#[test]
fn calculates_ratio() {
    assert!((MemoryBytes(5).ratio(MemoryBytes(10)) - 0.5).abs() < f64::EPSILON);
}
