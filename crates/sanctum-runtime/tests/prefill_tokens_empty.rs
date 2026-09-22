use sanctum_runtime::prefill::{PrefillError, TokenCount};

#[test]
fn rejects_empty_token_request() {
    assert_eq!(TokenCount::new(0), Err(PrefillError::EmptyRequest));
}
