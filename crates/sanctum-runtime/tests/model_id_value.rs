use sanctum_runtime::arbiter::ModelId;
#[test]
fn preserves_model_id() {
    assert_eq!(ModelId::new("coder").unwrap().as_str(), "coder");
}
