use sanctum_runtime::decision::Noul;
#[test]
fn bounds_logarithmic_score() {
    assert!(Noul::new(0.0).unwrap().positive_log_score().is_finite());
    assert!(Noul::new(1.0).unwrap().positive_log_score().abs() < f64::EPSILON);
}
