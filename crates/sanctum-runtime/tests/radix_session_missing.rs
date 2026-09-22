use sanctum_runtime::radix::*;
#[test]
fn reports_missing_session() {
    let mut c = RadixCache::new(CacheCapacity::default());
    assert!(c.remove_session(&SessionId::new("s").unwrap()).is_err());
}
