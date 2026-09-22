use sanctum_runtime::gateway::arithmetic_operator_count;
#[test]
fn counts_arithmetic_operators() {
    assert_eq!(arithmetic_operator_count("1+2*3"), 2);
}
