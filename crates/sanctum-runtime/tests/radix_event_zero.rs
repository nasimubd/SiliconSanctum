use sanctum_runtime::radix::CacheEventLog;
#[test]
fn rejects_zero_event_capacity() {
    assert!(CacheEventLog::new(0).is_err());
}
