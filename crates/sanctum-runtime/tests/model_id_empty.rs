use sanctum_runtime::arbiter::ModelId;
#[test]
fn rejects_empty_model_id() {
    assert!(ModelId::new("  ").is_err());
}
