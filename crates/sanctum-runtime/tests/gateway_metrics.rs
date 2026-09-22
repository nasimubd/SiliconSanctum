use sanctum_runtime::gateway::*;
#[test]
fn records_gateway_metrics() {
    let mut m = GatewayMetrics::default();
    m.record(GatewayPath::SystemOne);
    m.record(GatewayPath::DeterministicInterpreter);
    m.record(GatewayPath::HeavyReasoningModel);
    assert_eq!((m.system_one, m.deterministic, m.heavy), (1, 1, 1));
}
