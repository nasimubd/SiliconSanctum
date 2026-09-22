use sanctum_runtime::context::SourceLanguage;
#[test]
fn rejects_unknown_extension() {
    assert!(SourceLanguage::from_extension("java").is_err());
}
