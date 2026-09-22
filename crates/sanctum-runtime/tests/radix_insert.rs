use sanctum_runtime::radix::*;
#[test]
fn inserts_token_prefix() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let h = c.insert(&TokenSequence::new(vec![1, 2]).unwrap(), 64);
    assert!(c.contains_handle(h));
}
