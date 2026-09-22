use sanctum_runtime::radix::*;
#[test]
fn accounts_cache_bytes() {
    let mut c = RadixCache::new(CacheCapacity::default());
    c.insert(&TokenSequence::new(vec![1]).unwrap(), 64);
    assert_eq!(c.stats().bytes, 64);
}
