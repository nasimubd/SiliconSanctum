use sanctum_runtime::radix::*;
#[test]
fn reports_prefix_miss() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let s = TokenSequence::new(vec![9]).unwrap();
    assert!(!c.lookup(&s).hit());
}
