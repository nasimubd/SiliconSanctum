use sanctum_runtime::speculative::{SpeculativeError, VerifyWidth};
#[test]
fn rejects_zero_width() {
    assert_eq!(
        VerifyWidth::new(0),
        Err(SpeculativeError::ZeroValue("verify width"))
    );
}
