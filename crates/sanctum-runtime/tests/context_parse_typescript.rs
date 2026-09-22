use sanctum_runtime::context::*;
#[test]
fn parses_typescript_source() {
    let d = SourceDocument::new("x.ts", b"function f(): number { return 1; }".to_vec()).unwrap();
    assert_eq!(ParsedDocument::parse(d).unwrap().root().kind(), "program");
}
