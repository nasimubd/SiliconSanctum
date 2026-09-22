use sanctum_runtime::radix::validate_model_name;
#[test]
fn accepts_llama_prefix_cache() {
    assert!(validate_model_name("llama-3").is_ok());
}
