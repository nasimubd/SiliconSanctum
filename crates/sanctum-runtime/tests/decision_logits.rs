use sanctum_runtime::decision::InferenceLogits;
#[test]
fn rejects_invalid_inference_logits() {
    assert!(InferenceLogits::new(vec![]).is_err());
    assert!(InferenceLogits::new(vec![f32::INFINITY]).is_err());
}
