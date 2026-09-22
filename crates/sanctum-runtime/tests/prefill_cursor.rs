use sanctum_runtime::prefill::{ChunkCursor, PrefillStep, TokenCount, partition_tokens};

#[test]
fn traverses_chunks_in_order() {
    let chunks = partition_tokens(TokenCount::new(700).unwrap(), PrefillStep::Tokens512);
    let mut cursor = ChunkCursor::new(chunks);
    assert_eq!(cursor.remaining(), 2);
    assert_eq!(cursor.advance().unwrap().offset, 0);
    assert_eq!(cursor.advance().unwrap().offset, 512);
    assert_eq!(cursor.remaining(), 0);
}
