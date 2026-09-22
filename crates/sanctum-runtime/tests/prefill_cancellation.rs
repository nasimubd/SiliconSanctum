use sanctum_runtime::prefill::CancellationToken;

#[test]
fn shares_cancellation_state() {
    let token = CancellationToken::default();
    let observer = token.clone();
    assert!(!observer.is_cancelled());
    token.cancel();
    assert!(observer.is_cancelled());
}
