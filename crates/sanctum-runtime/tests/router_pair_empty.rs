use sanctum_runtime::router::ModelPair;
#[test]
fn rejects_empty_model_pair() {
    assert!(ModelPair::new("", "target").is_err());
}
