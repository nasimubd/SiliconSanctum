use sanctum_runtime::gateway::GatewayPrompt;
#[test]
fn normalizes_gateway_prompt() {
    assert_eq!(GatewayPrompt::new("HELLO").unwrap().normalized(), "hello");
}
