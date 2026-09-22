use sanctum_runtime::prefill::{PrefillStep, TokenCount, partition_tokens};

#[test]
fn partitions_with_512_token_step() {
    let chunks = partition_tokens(TokenCount::new(1024).unwrap(), PrefillStep::Tokens512);
    assert_eq!(chunks.len(), 2);
    assert!(chunks.iter().all(|chunk| chunk.len() == 512));
}
