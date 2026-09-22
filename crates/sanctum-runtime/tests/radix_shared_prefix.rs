use sanctum_runtime::radix::*;
#[test]
fn extracts_shared_prefix() {
    let a = TokenSequence::new(vec![1, 2, 3]).unwrap();
    let b = TokenSequence::new(vec![1, 2, 4]).unwrap();
    assert_eq!(shared_prefix(&a, &b).unwrap().tokens(), [1, 2]);
}
