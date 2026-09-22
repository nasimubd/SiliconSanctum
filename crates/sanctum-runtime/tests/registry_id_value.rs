use sanctum_runtime::registry::RegistryModelId;
#[test]
fn preserves_registry_id() {
    assert_eq!(RegistryModelId::new("coder").unwrap().as_str(), "coder");
}
