use sanctum_runtime::arbiter::{
    ArbiterAction, ArbiterPolicy, DecisionEngine, MemoryArbiter, MemorySnapshot, StaticTelemetry,
};
#[test]
fn executes_daemon_tick() {
    let sample = MemorySnapshot::new(0, 100, 0);
    let mut daemon = MemoryArbiter::new(
        StaticTelemetry::new(sample),
        DecisionEngine::new(ArbiterPolicy::default()),
        sample,
        4,
    )
    .unwrap();
    assert_eq!(daemon.tick().unwrap().action, ArbiterAction::Hold);
}
