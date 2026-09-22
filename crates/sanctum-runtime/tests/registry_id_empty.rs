use sanctum_runtime::registry::RegistryModelId;
#[test]
fn rejects_empty_registry_id() {
    assert!(RegistryModelId::new(" ").is_err());
}
