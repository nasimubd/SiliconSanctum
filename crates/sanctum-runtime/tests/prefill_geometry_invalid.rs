use sanctum_runtime::prefill::{ModelGeometry, PrefillError};

#[test]
fn rejects_incomplete_model_geometry() {
    assert_eq!(
        ModelGeometry::new(0, 8, 128),
        Err(PrefillError::InvalidGeometry)
    );
}
