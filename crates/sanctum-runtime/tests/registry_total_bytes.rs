use sanctum_runtime::registry::*;
#[test]
fn calculates_total_bytes() {
    let value = ModelEntry::new(
        RegistryModelId::new("m").unwrap(),
        ModelBackend::MlxLm,
        ModelFormat::Mlx,
        Quantization::Q4,
        10,
        2,
        ContextLadder::new(vec![4]).unwrap(),
    )
    .unwrap();
    assert_eq!(value.total_bytes_for(4), 18);
}
