use sanctum_runtime::context::*;
#[test]
fn prunes_cpp_body() {
    let d = SourceDocument::new("x.cpp", b"int f() { return 999; }".to_vec()).unwrap();
    let s = summarize_document(d, PruningPolicy::default()).unwrap();
    assert!(s.text.contains("int f"));
    assert!(!s.text.contains("999"));
}
