use sanctum_runtime::arbiter::{
    ArbiterAction, ArbiterPolicy, DecisionEngine, MemorySnapshot, PressureLevel,
};
#[test]
fn holds_within_budget() {
    let result = DecisionEngine::new(ArbiterPolicy::default()).evaluate(
        MemorySnapshot::new(0, 1, 0),
        PressureLevel::Normal,
        false,
    );
    assert_eq!(result.action, ArbiterAction::Hold);
}
