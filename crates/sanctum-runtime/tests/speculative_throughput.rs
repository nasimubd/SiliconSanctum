use sanctum_runtime::speculative::TimingSample;
#[test]
fn calculates_rate() {
    let rate = TimingSample {
        tokens: 20,
        elapsed_seconds: 2.0,
    }
    .tokens_per_second();
    assert!((rate - 10.0).abs() < f64::EPSILON);
}
