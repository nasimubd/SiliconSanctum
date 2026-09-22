use sanctum_runtime::gateway::*;
#[test]
fn escalates_temporal_to_heavy_model() {
    let g = SystemOneGateway::default();
    let d = g.decide(&GatewayPrompt::new("before yesterday").unwrap());
    assert_eq!(d.path, GatewayPath::HeavyReasoningModel);
}
