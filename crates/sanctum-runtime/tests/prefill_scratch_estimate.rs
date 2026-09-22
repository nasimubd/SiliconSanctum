use sanctum_runtime::prefill::{ModelGeometry, PrefillStep, estimate_scratch_bytes};

#[test]
fn estimates_full_precision_scratch() {
    let geometry = ModelGeometry::new(2, 2, 4).unwrap();
    assert_eq!(
        estimate_scratch_bytes(PrefillStep::Tokens512, geometry),
        64 * 512
    );
}
