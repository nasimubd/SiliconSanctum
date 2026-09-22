use sanctum_runtime::router::FixedAxisEvaluator;
#[test]
fn rejects_invalid_fixed_evaluator() {
    assert!(FixedAxisEvaluator::new(f64::NAN, std::time::Duration::ZERO).is_err());
}
