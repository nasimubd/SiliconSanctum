use sanctum_runtime::tui::Utilization;
#[test]
fn rejects_excess() {
    assert!(Utilization::new(1.1).is_err());
}
