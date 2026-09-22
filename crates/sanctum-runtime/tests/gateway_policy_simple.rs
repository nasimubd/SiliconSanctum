use sanctum_runtime::gateway::*;
#[test]
fn forwards_simple_prompt_to_system_one() {
    let g = SystemOneGateway::default();
    let d = g.decide(&GatewayPrompt::new("open settings").unwrap());
    assert_eq!(d.path, GatewayPath::SystemOne);
}
