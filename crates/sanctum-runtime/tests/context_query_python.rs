use sanctum_runtime::context::*;
#[test]
fn compiles_python_query() {
    assert!(compile_query(SourceLanguage::Python, &python_query()).is_ok());
}
