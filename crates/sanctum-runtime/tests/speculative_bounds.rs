use sanctum_runtime::speculative::{SpeculativeError, VerifyWidth, VerifyWidthBounds};
#[test]
fn rejects_inverted_bounds() {
    assert_eq!(
        VerifyWidthBounds::new(VerifyWidth::new(8).unwrap(), VerifyWidth::new(4).unwrap()),
        Err(SpeculativeError::InvalidWidthBounds {
            minimum: 8,
            maximum: 4
        })
    );
}
