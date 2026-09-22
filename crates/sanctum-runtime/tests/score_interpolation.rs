use sanctum_runtime::decision::*;
#[test]
fn interpolates_ordinal_score() {
    let rubric = ScoreRubric::new(vec![
        RubricPoint::new(0.0, 0.0).unwrap(),
        RubricPoint::new(1.0, 10.0).unwrap(),
    ])
    .unwrap();
    assert!((rubric.interpolate(0.5).value() - 5.0).abs() < f64::EPSILON);
}
