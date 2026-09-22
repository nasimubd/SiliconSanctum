use sanctum_runtime::decision::Noul;
#[test]
fn calibrates_neutral_logit() {
    assert!((Noul::from_logit(0.0).unwrap().probability() - 0.5).abs() < f64::EPSILON);
}
