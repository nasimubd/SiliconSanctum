use sanctum_runtime::gateway::*;
#[test]
fn rejects_empty_evidence() {
    assert!(Evidence::new(JaggednessKind::Counting, "").is_err());
}
