use sanctum_runtime::context::*;
#[test]
fn resolves_typescript_extension() {
    assert_eq!(
        SourceLanguage::from_extension("tsx").unwrap(),
        SourceLanguage::TypeScript
    );
}
