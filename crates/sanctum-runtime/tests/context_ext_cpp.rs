use sanctum_runtime::context::*;
#[test]
fn resolves_cpp_extension() {
    assert_eq!(
        SourceLanguage::from_extension("hpp").unwrap(),
        SourceLanguage::Cpp
    );
}
