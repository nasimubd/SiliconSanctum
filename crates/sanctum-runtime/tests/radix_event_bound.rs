use sanctum_runtime::radix::*;
#[test]
fn bounds_cache_events() {
    let mut l = CacheEventLog::new(1).unwrap();
    let e = CacheEvent {
        kind: CacheEventKind::Miss,
        handle: None,
        token_count: 1,
    };
    l.push(e);
    l.push(e);
    assert_eq!(l.entries().len(), 1);
}
