use sanctum_runtime::context::*;
#[test]
fn parses_rust_source() {
    let d = SourceDocument::new("x.rs", b"fn f() -> i32 { 1 }".to_vec()).unwrap();
    assert_eq!(
        ParsedDocument::parse(d).unwrap().root().kind(),
        "source_file"
    );
}
