use sanctum_runtime::router::*;
#[test]
fn calculates_fanout_minimum() {
    let r = FanoutResult {
        intent: AxisEvaluation::new(EvaluationAxis::Intent, 0.9).unwrap(),
        tooling: AxisEvaluation::new(EvaluationAxis::Tooling, 0.8).unwrap(),
        complexity: AxisEvaluation::new(EvaluationAxis::Complexity, 0.7).unwrap(),
    };
    assert!((r.minimum_confidence() - 0.7).abs() < f64::EPSILON);
}
