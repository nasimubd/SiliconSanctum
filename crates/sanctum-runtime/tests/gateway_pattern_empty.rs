use sanctum_runtime::gateway::PatternSet;
#[test]
fn rejects_empty_patterns() {
    assert!(PatternSet::new(vec![]).is_err());
}
