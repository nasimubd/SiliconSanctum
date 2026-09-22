use sanctum_runtime::radix::*;
#[test]
fn reports_cache_eviction() {
    let mut c = RadixCache::new(CacheCapacity::default());
    c.insert(&TokenSequence::new(vec![1]).unwrap(), 64);
    let o = c.evict_all();
    assert_eq!((o.entries_removed, o.bytes_reclaimed), (1, 64));
}
