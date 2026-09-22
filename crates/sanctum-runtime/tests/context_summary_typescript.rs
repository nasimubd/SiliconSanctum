use sanctum_runtime::context::*;
#[test]
fn prunes_typescript_body() {
    let d = SourceDocument::new("x.ts", b"function f(): number { return 999; }".to_vec()).unwrap();
    let s = summarize_document(d, PruningPolicy::default()).unwrap();
    assert!(s.text.contains("function f"));
    assert!(!s.text.contains("999"));
}
