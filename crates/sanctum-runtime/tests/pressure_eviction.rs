use sanctum_runtime::arbiter::PressureLevel;
#[test]
fn pressure_requires_eviction() {
    assert!(PressureLevel::Warning.requires_eviction());
    assert!(PressureLevel::Critical.requires_eviction());
    assert!(!PressureLevel::Normal.requires_eviction());
}
