use sanctum_runtime::context::*;
#[test]
fn prunes_rust_body() {
    let d = SourceDocument::new("x.rs", b"fn f() -> i32 { 999 }".to_vec()).unwrap();
    let s = summarize_document(d, PruningPolicy::default()).unwrap();
    assert!(s.text.contains("fn f"));
    assert!(!s.text.contains("999"));
}
