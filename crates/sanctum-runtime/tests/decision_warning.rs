use sanctum_runtime::arbiter::{
    ArbiterAction, ArbiterPolicy, DecisionEngine, MemorySnapshot, PressureLevel,
};
#[test]
fn evicts_on_warning_pressure() {
    let result = DecisionEngine::new(ArbiterPolicy::default()).evaluate(
        MemorySnapshot::new(0, 1, 0),
        PressureLevel::Warning,
        false,
    );
    assert_eq!(result.action, ArbiterAction::Evict);
}
