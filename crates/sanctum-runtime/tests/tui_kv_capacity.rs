use sanctum_runtime::tui::KvResidency;
#[test]
fn rejects_excess_occupancy() {
    assert!(KvResidency::new(5, 4).is_err());
}
