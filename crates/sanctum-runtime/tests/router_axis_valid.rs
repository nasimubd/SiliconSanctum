use sanctum_runtime::router::*;
#[test]
fn preserves_axis_confidence() {
    assert!(
        (AxisEvaluation::new(EvaluationAxis::Tooling, 0.8)
            .unwrap()
            .confidence()
            - 0.8)
            .abs()
            < f64::EPSILON
    );
}
