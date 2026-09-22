use sanctum_runtime::context::*;
#[test]
fn resolves_rust_extension() {
    assert_eq!(
        SourceLanguage::from_extension(".rs").unwrap(),
        SourceLanguage::Rust
    );
}
