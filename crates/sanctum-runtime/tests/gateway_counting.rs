use sanctum_runtime::gateway::*;
#[test]
fn detects_counting_prompt() {
    let r =
        JaggednessFilter::default().inspect(&GatewayPrompt::new("how many files exist").unwrap());
    assert!(r.contains(JaggednessKind::Counting));
}
