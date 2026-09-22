use sanctum_runtime::context::SourceRange;
#[test]
fn rejects_inverted_source_range() {
    assert!(SourceRange::new(2, 1).is_err());
}
