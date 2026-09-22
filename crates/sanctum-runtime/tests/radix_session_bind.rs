use sanctum_runtime::radix::*;
#[test]
fn binds_cache_session() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let id = SessionId::new("s").unwrap();
    c.bind_session(id.clone(), TokenSequence::new(vec![1]).unwrap());
    assert!(c.session(&id).is_some());
}
