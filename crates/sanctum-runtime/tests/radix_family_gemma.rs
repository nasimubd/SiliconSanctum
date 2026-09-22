use sanctum_runtime::radix::*;
#[test]
fn infers_gemma_architecture() {
    assert_eq!(
        infer_architecture_family("gemma-3"),
        ArchitectureFamily::Gemma
    );
}
