use sanctum_runtime::gateway::GatewayPrompt;
#[test]
fn rejects_empty_gateway_prompt() {
    assert!(GatewayPrompt::new(" ").is_err());
}
