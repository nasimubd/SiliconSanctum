use sanctum_runtime::gateway::*;
#[test]
fn detects_temporal_prompt() {
    let r = JaggednessFilter::default()
        .inspect(&GatewayPrompt::new("what happened before launch").unwrap());
    assert!(r.contains(JaggednessKind::RelativeTemporal));
}
