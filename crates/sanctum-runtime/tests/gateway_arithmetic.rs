use sanctum_runtime::gateway::*;
#[test]
fn detects_multihop_arithmetic() {
    let r = JaggednessFilter::default()
        .inspect(&GatewayPrompt::new("calculate this then sum it").unwrap());
    assert!(r.contains(JaggednessKind::MultiHopArithmetic));
}
