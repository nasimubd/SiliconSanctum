use sanctum_runtime::context::*;
#[test]
fn reports_context_compression() {
    let d = SourceDocument::new(
        "x.rs",
        b"fn f() { let payload = [0; 1000]; println!(\"{:?}\", payload); }".to_vec(),
    )
    .unwrap();
    let s = summarize_document(d, PruningPolicy::default()).unwrap();
    assert!(s.metrics.summary_bytes < s.metrics.original_bytes);
    assert!(s.metrics.ratio() > 0.0);
}
