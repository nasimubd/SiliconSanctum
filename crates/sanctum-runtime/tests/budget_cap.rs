use sanctum_runtime::arbiter::{ArbiterPolicy, BudgetRequest, MemorySnapshot, calculate_budget};
#[test]
fn caps_kv_budget() {
    let budget = calculate_budget(
        MemorySnapshot::new(20, 80, 0),
        ArbiterPolicy::new(100, 10, 10).unwrap(),
        BudgetRequest::new(30, 100),
    );
    assert_eq!(budget.kv_cache_bytes, 40);
}
