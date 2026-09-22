use sanctum_runtime::arbiter::ModelResidency;
#[test]
fn classifies_resident_memory() {
    assert!(ModelResidency::Resident.occupies_memory());
    assert!(!ModelResidency::Evicted.occupies_memory());
}
