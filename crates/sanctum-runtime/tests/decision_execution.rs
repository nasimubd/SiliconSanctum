use sanctum_runtime::decision::*;
#[test]
fn measures_fixture_execution() {
    let input = InferenceInput::new(vec![1.0]).unwrap();
    let mut executor = FixtureExecutor::new(
        InferenceLogits::new(vec![0.0]).unwrap(),
        MIN_DECISION_LATENCY,
    );
    let result = execute_measured(&mut executor, &input, LatencyBudget::default()).unwrap();
    assert_eq!(result.logits.values(), [0.0]);
    assert_eq!(executor.invocations(), 1);
}
