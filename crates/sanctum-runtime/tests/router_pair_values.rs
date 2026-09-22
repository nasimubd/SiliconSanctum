use sanctum_runtime::router::ModelPair;
#[test]
fn preserves_model_pair() {
    let v = ModelPair::new("draft", "target").unwrap();
    assert_eq!((v.draft(), v.target()), ("draft", "target"));
}
