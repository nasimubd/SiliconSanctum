use sanctum_runtime::gateway::GatewayEventLog;
#[test]
fn rejects_zero_event_capacity() {
    assert!(GatewayEventLog::new(0).is_err());
}
