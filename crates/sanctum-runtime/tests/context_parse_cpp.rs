use sanctum_runtime::context::*;
#[test]
fn parses_cpp_source() {
    let d = SourceDocument::new("x.cpp", b"int f() { return 1; }".to_vec()).unwrap();
    assert_eq!(
        ParsedDocument::parse(d).unwrap().root().kind(),
        "translation_unit"
    );
}
