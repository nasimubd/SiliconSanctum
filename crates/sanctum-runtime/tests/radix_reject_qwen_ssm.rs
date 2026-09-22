use sanctum_runtime::radix::validate_model_name;
#[test]
fn rejects_qwen_ssm_prefix_cache() {
    assert!(validate_model_name("qwen-3.5-ssm").is_err());
}
