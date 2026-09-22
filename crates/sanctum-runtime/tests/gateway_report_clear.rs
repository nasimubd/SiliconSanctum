use sanctum_runtime::gateway::JaggednessReport;
#[test]
fn starts_with_clear_report() {
    assert!(!JaggednessReport::clear().is_jagged());
}
