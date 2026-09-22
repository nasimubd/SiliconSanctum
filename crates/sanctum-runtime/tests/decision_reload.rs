use sanctum_runtime::arbiter::{
    ArbiterAction, ArbiterPolicy, DecisionEngine, MemorySnapshot, PressureLevel,
};
#[test]
fn reloads_after_recovery() {
    let policy = ArbiterPolicy::new(100, 10, 10).unwrap();
    let result = DecisionEngine::new(policy).evaluate(
        MemorySnapshot::new(70, 30, 0),
        PressureLevel::Normal,
        true,
    );
    assert_eq!(result.action, ArbiterAction::Reload);
}
