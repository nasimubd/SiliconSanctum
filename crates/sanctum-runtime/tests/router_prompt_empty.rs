use sanctum_runtime::router::RoutingPrompt;
#[test]
fn rejects_empty_prompt() {
    assert!(RoutingPrompt::new(" ").is_err());
}
