use sanctum_runtime::decision::*;
#[test]
fn validates_latency_boundaries() {
    assert!(LatencyBudget::new(MIN_DECISION_LATENCY, MAX_DECISION_LATENCY).is_ok());
    assert!(
        LatencyBudget::new(std::time::Duration::from_millis(69), MAX_DECISION_LATENCY).is_err()
    );
}
