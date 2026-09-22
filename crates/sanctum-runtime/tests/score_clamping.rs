use sanctum_runtime::decision::*;
#[test]
fn clamps_ordinal_score() {
    let rubric = ScoreRubric::new(vec![
        RubricPoint::new(0.0, 2.0).unwrap(),
        RubricPoint::new(1.0, 8.0).unwrap(),
    ])
    .unwrap();
    assert!((rubric.interpolate(-1.0).value() - 2.0).abs() < f64::EPSILON);
    assert!((rubric.interpolate(2.0).value() - 8.0).abs() < f64::EPSILON);
}
