use sanctum_runtime::gateway::requires_arithmetic_bypass;
#[test]
fn requires_complex_arithmetic_bypass() {
    assert!(requires_arithmetic_bypass("1+2*3"));
    assert!(!requires_arithmetic_bypass("1+2"));
}
