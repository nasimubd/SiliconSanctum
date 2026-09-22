use sanctum_runtime::speculative::{AdaptationPolicy, VerifyWidth, VerifyWidthBounds};
#[test]
fn minimizes_at_exhaustion() {
    let p = AdaptationPolicy {
        widths: VerifyWidthBounds::new(VerifyWidth::new(1).unwrap(), VerifyWidth::new(16).unwrap())
            .unwrap(),
        medium_pressure: 0.5,
        high_pressure: 0.9,
    };
    assert_eq!(p.select(1.0).get(), 1);
}
