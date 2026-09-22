use sanctum_runtime::arbiter::{ModelFootprint, ModelId};
#[test]
fn totals_model_memory() {
    let value = ModelFootprint::new(ModelId::new("m").unwrap(), 10, 5, 0).unwrap();
    assert_eq!(value.total_bytes(), 15);
}
