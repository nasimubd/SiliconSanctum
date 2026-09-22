use sanctum_runtime::context::*;
#[test]
fn parses_python_source() {
    let d = SourceDocument::new("x.py", b"def f():\n    return 1\n".to_vec()).unwrap();
    assert_eq!(ParsedDocument::parse(d).unwrap().root().kind(), "module");
}
