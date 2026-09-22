use sanctum_runtime::context::SourceDocument;
#[test]
fn rejects_empty_source_path() {
    assert!(SourceDocument::new("", b"x".to_vec()).is_err());
}
