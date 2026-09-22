use sanctum_runtime::context::*;
#[test]
fn resolves_python_extension() {
    assert_eq!(
        SourceLanguage::from_extension("py").unwrap(),
        SourceLanguage::Python
    );
}
