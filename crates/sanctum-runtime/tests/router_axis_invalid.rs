use sanctum_runtime::router::*;
#[test]
fn rejects_invalid_axis_confidence() {
    assert!(AxisEvaluation::new(EvaluationAxis::Intent, 1.1).is_err());
}
