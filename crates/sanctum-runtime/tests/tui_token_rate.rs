use sanctum_runtime::tui::TokenRate;
#[test]
fn rejects_nan() {
    assert!(TokenRate::new(f64::NAN).is_err());
}
