use sanctum_runtime::gateway::*;
#[test]
fn escalates_counting_to_interpreter() {
    let g = SystemOneGateway::default();
    let d = g.decide(&GatewayPrompt::new("count the files").unwrap());
    assert_eq!(d.path, GatewayPath::DeterministicInterpreter);
}
