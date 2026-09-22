use sanctum_runtime::gateway::*;
#[test]
fn bounds_gateway_events() {
    let mut l = GatewayEventLog::new(1).unwrap();
    l.push(GatewayEvent::Inspected);
    l.push(GatewayEvent::Forwarded);
    assert_eq!(l.entries().len(), 1);
}
