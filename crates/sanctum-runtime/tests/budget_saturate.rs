use sanctum_runtime::arbiter::{ArbiterPolicy, BudgetRequest, MemorySnapshot, calculate_budget};
#[test]
fn saturates_exhausted_budget() {
    let budget = calculate_budget(
        MemorySnapshot::new(100, 0, 0),
        ArbiterPolicy::new(100, 10, 10).unwrap(),
        BudgetRequest::new(30, 20),
    );
    assert_eq!(budget.total_allocated(), 0);
}
