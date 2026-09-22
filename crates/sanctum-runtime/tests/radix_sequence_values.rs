use sanctum_runtime::radix::TokenSequence;
#[test]
fn preserves_token_sequence() {
    let v = TokenSequence::new(vec![1, 2]).unwrap();
    assert_eq!(v.tokens(), [1, 2]);
}
