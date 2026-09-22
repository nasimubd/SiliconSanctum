use sanctum_runtime::gateway::contains_digit;
#[test]
fn detects_numeric_tokens() {
    assert!(contains_digit("value 42"));
    assert!(!contains_digit("value"));
}
