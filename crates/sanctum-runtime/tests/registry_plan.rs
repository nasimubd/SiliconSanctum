use sanctum_runtime::registry::*;
#[test]
fn selects_largest_affordable_context() {
    let value = ModelEntry::new(
        RegistryModelId::new("m").unwrap(),
        ModelBackend::MlxLm,
        ModelFormat::Mlx,
        Quantization::Q4,
        10,
        2,
        ContextLadder::new(vec![4, 8]).unwrap(),
    )
    .unwrap();
    assert_eq!(value.plan_with_budget(20).unwrap().context_tokens, 4);
}
