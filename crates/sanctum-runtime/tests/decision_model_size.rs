use sanctum_runtime::decision::*;
#[test]
fn enforces_model_size_ceiling() {
    let id = DecisionModelId::new("openJev-verdict-2.0").unwrap();
    assert!(DecisionModelMetadata::new(id, MAX_DECISION_PARAMETERS).is_err());
}
