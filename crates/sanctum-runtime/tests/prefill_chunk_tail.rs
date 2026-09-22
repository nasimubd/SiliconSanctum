use sanctum_runtime::prefill::{PrefillStep, TokenCount, partition_tokens};

#[test]
fn preserves_tail_chunk() {
    let chunks = partition_tokens(TokenCount::new(1300).unwrap(), PrefillStep::Tokens512);
    assert_eq!(chunks.last().unwrap().len(), 276);
    assert_eq!(chunks.last().unwrap().end, 1300);
}
