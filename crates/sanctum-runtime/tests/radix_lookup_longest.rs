use sanctum_runtime::radix::*;
#[test]
fn traverses_longest_prefix() {
    let mut c = RadixCache::new(CacheCapacity::default());
    c.insert(&TokenSequence::new(vec![1, 2]).unwrap(), 64);
    let m = c.lookup(&TokenSequence::new(vec![1, 2, 3]).unwrap());
    assert_eq!((m.matched_tokens, m.remaining_tokens), (2, 1));
}
