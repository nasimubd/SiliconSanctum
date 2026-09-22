use sanctum_runtime::arbiter::{
    ArbiterAction, ArbiterPolicy, DecisionEngine, MemorySnapshot, PressureLevel,
};
#[test]
fn evicts_when_swap_detected() {
    let result = DecisionEngine::new(ArbiterPolicy::default()).evaluate(
        MemorySnapshot::new(0, 1, 1),
        PressureLevel::Normal,
        false,
    );
    assert_eq!(result.action, ArbiterAction::Evict);
}
