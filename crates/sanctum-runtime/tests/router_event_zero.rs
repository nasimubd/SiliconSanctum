use sanctum_runtime::router::RouterEventLog;
#[test]
fn rejects_zero_event_capacity() {
    assert!(RouterEventLog::new(0).is_err());
}
