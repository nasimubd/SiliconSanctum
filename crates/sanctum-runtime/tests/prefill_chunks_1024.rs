use sanctum_runtime::prefill::{PrefillStep, TokenCount, partition_tokens};

#[test]
fn partitions_with_1024_token_step() {
    let chunks = partition_tokens(TokenCount::new(2048).unwrap(), PrefillStep::Tokens1024);
    assert_eq!(chunks.len(), 2);
    assert!(chunks.iter().all(|chunk| chunk.len() == 1024));
}
