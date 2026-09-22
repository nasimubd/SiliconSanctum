use sanctum_runtime::registry::*;
#[test]
fn rejects_zero_model_weights() {
    let result = ModelEntry::new(
        RegistryModelId::new("m").unwrap(),
        ModelBackend::MlxLm,
        ModelFormat::Mlx,
        Quantization::Q4,
        0,
        1,
        ContextLadder::new(vec![1]).unwrap(),
    );
    assert!(result.is_err());
}
