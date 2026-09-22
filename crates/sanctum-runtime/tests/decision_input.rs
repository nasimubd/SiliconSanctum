use sanctum_runtime::decision::InferenceInput;
#[test]
fn rejects_invalid_inference_input() {
    assert!(InferenceInput::new(vec![]).is_err());
    assert!(InferenceInput::new(vec![f32::NAN]).is_err());
}
