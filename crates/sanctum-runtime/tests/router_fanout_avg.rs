use sanctum_runtime::router::*;
#[test]
fn calculates_fanout_average() {
    let r = FanoutResult {
        intent: AxisEvaluation::new(EvaluationAxis::Intent, 0.6).unwrap(),
        tooling: AxisEvaluation::new(EvaluationAxis::Tooling, 0.6).unwrap(),
        complexity: AxisEvaluation::new(EvaluationAxis::Complexity, 0.6).unwrap(),
    };
    assert!((r.average_confidence() - 0.6).abs() < f64::EPSILON);
}
