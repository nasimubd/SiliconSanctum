use sanctum_runtime::radix::AttentionArchitecture;
#[test]
fn permits_only_full_attention() {
    assert!(AttentionArchitecture::Full.supports_prefix_cache());
    assert!(!AttentionArchitecture::Hybrid.supports_prefix_cache());
}
