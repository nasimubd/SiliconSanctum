use sanctum_runtime::tui::DashboardTab;
#[test]
fn cycles_tabs() {
    assert_eq!(
        DashboardTab::Overview.next().next().next(),
        DashboardTab::Overview
    );
}
