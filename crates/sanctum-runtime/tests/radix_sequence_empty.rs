use sanctum_runtime::radix::TokenSequence;
#[test]
fn rejects_empty_token_sequence() {
    assert!(TokenSequence::new(vec![]).is_err());
}
