use sanctum_runtime::tui::DashboardError;
#[test]
fn displays_error() {
    assert_eq!(DashboardError::ZeroDimension.to_string(), "ZeroDimension");
}
