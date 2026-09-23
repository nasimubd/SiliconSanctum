use sanctum_runtime::tui::{MetricSeverity, severity};
#[test]
fn classifies_boundaries() {
    assert_eq!(severity(0.74), MetricSeverity::Normal);
    assert_eq!(severity(0.75), MetricSeverity::Warning);
    assert_eq!(severity(0.9), MetricSeverity::Critical);
}
