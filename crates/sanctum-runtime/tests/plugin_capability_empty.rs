use sanctum_runtime::plugin::Capability;
#[test]
fn rejects_empty_capability() {
    assert!(Capability::new(" ").is_err());
}
