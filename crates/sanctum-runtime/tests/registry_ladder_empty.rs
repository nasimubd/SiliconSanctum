use sanctum_runtime::registry::ContextLadder;
#[test]
fn rejects_empty_context_ladder() {
    assert!(ContextLadder::new(vec![]).is_err());
}
