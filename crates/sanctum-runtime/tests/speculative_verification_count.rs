use sanctum_runtime::speculative::Verification;
#[test]
fn counts_fallback() {
    assert_eq!(
        Verification {
            accepted: 2,
            fallback_token: Some(3)
        }
        .emitted_tokens(),
        3
    );
}
