use sanctum_runtime::context::SourceDocument;
#[test]
fn rejects_empty_source_bytes() {
    assert!(SourceDocument::new("x.py", vec![]).is_err());
}
