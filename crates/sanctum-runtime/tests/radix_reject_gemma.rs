use sanctum_runtime::radix::validate_model_name;
#[test]
fn rejects_gemma_prefix_cache() {
    assert!(validate_model_name("gemma-3").is_err());
}
