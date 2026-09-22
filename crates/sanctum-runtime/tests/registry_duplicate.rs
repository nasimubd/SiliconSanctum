use sanctum_runtime::registry::*;
fn entry() -> ModelEntry {
    ModelEntry::new(
        RegistryModelId::new("m").unwrap(),
        ModelBackend::MlxLm,
        ModelFormat::Mlx,
        Quantization::Q4,
        10,
        2,
        ContextLadder::new(vec![4]).unwrap(),
    )
    .unwrap()
}
#[test]
fn rejects_duplicate_models() {
    let mut registry = ModelRegistry::new();
    registry.insert(entry()).unwrap();
    assert!(registry.insert(entry()).is_err());
}
