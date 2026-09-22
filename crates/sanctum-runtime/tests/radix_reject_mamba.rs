use sanctum_runtime::radix::validate_model_name;
#[test]
fn rejects_mamba_prefix_cache() {
    assert!(validate_model_name("mamba-2").is_err());
}
