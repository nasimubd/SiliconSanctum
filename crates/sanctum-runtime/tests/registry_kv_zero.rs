use sanctum_runtime::registry::*;
#[test]
fn rejects_zero_kv_rate() {
    let result = ModelEntry::new(
        RegistryModelId::new("m").unwrap(),
        ModelBackend::MlxLm,
        ModelFormat::Mlx,
        Quantization::Q4,
        1,
        0,
        ContextLadder::new(vec![1]).unwrap(),
    );
    assert!(result.is_err());
}
