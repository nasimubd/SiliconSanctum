use sanctum_runtime::gateway::*;
#[test]
fn reports_bypass_state() {
    let d = SystemOneGateway::default().decide(&GatewayPrompt::new("count items").unwrap());
    assert!(d.bypassed());
}
