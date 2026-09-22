use sanctum_runtime::arbiter::{ModelFootprint, ModelId};
#[test]
fn rejects_zero_weight_footprint() {
    assert!(ModelFootprint::new(ModelId::new("m").unwrap(), 0, 1, 0).is_err());
}
