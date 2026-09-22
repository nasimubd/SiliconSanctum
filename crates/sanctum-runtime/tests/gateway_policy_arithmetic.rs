use sanctum_runtime::gateway::*;
#[test]
fn escalates_arithmetic_to_interpreter() {
    let g = SystemOneGateway::default();
    let d = g.decide(&GatewayPrompt::new("calculate then sum").unwrap());
    assert_eq!(d.path, GatewayPath::DeterministicInterpreter);
}
