use sanctum_runtime::radix::*;
#[test]
fn reuses_existing_handle() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let s = TokenSequence::new(vec![1, 2]).unwrap();
    assert_eq!(c.insert(&s, 64), c.insert(&s, 64));
}
