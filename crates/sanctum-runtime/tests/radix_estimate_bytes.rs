use sanctum_runtime::radix::estimated_token_bytes;
#[test]
fn estimates_token_bytes() {
    assert_eq!(estimated_token_bytes(512, 16), 8192);
}
