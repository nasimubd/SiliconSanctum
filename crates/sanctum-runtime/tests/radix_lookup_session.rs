use sanctum_runtime::radix::*;
#[test]
fn looks_up_session_prefix() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let id = SessionId::new("s").unwrap();
    let s = TokenSequence::new(vec![1]).unwrap();
    c.insert(&s, 4);
    c.bind_session(id.clone(), s);
    assert!(c.lookup_session(&id).unwrap().hit());
}
