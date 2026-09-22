use sanctum_runtime::context::*;
#[test]
fn compiles_rust_query() {
    assert!(compile_query(SourceLanguage::Rust, &rust_query()).is_ok());
}
