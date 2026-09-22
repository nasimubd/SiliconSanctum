use sanctum_runtime::gateway::*;
#[test]
fn permits_simple_prompt() {
    let r = JaggednessFilter::default().inspect(&GatewayPrompt::new("open settings").unwrap());
    assert!(!r.is_jagged());
}
