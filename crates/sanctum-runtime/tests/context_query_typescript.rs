use sanctum_runtime::context::*;
#[test]
fn compiles_typescript_query() {
    assert!(compile_query(SourceLanguage::TypeScript, &typescript_query()).is_ok());
}
