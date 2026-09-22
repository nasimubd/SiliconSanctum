use sanctum_runtime::router::RoutingPrompt;
#[test]
fn preserves_prompt() {
    assert_eq!(RoutingPrompt::new("hello").unwrap().as_str(), "hello");
}
