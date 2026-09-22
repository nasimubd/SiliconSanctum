use sanctum_runtime::registry::ContextLadder;
#[test]
fn rejects_unordered_context_ladder() {
    assert!(ContextLadder::new(vec![4096, 2048]).is_err());
}
