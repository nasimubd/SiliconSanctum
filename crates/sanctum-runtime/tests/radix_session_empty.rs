use sanctum_runtime::radix::SessionId;
#[test]
fn rejects_empty_session() {
    assert!(SessionId::new(" ").is_err());
}
