use sanctum_runtime::radix::*;
#[test]
fn reports_prefix_hit() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let s = TokenSequence::new(vec![1, 2]).unwrap();
    c.insert(&s, 64);
    assert!(c.lookup(&s).hit());
}
