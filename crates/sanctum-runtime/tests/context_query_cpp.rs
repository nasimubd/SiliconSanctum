use sanctum_runtime::context::*;
#[test]
fn compiles_cpp_query() {
    assert!(compile_query(SourceLanguage::Cpp, &cpp_query()).is_ok());
}
