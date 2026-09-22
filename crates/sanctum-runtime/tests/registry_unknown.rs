use sanctum_runtime::registry::ModelRegistry;
#[test]
fn reports_unknown_models() {
    assert!(ModelRegistry::new().get("missing").is_err());
}
