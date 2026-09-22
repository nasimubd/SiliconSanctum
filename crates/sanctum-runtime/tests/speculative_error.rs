use sanctum_runtime::speculative::SpeculativeError;
#[test]
fn displays_error_variant() {
    assert_eq!(SpeculativeError::Cancelled.to_string(), "Cancelled");
}
