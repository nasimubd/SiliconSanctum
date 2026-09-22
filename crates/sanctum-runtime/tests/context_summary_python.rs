use sanctum_runtime::context::*;
#[test]
fn prunes_python_body() {
    let d = SourceDocument::new("x.py", b"def f():\n    return 999\n".to_vec()).unwrap();
    let s = summarize_document(d, PruningPolicy::default()).unwrap();
    assert!(s.text.contains("def f"));
    assert!(!s.text.contains("999"));
}
