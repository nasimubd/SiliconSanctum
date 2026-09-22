use sanctum_runtime::arbiter::EventLog;
#[test]
fn rejects_zero_event_capacity() {
    assert!(EventLog::new(0).is_err());
}
