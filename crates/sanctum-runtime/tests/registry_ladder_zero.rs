use sanctum_runtime::registry::ContextLadder;
#[test]
fn rejects_zero_context_level() {
    assert!(ContextLadder::new(vec![0]).is_err());
}
