use sanctum_runtime::radix::*;
#[test]
fn extends_session_prefix() {
    let mut c = RadixCache::new(CacheCapacity::default());
    let id = SessionId::new("s").unwrap();
    c.bind_session(id.clone(), TokenSequence::new(vec![1]).unwrap());
    c.extend_session(&id, &[2]).unwrap();
    assert_eq!(c.session(&id).unwrap().tokens(), [1, 2]);
}
