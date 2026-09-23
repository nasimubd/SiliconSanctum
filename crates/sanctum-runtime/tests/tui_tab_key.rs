use crossterm::event::KeyCode;
use sanctum_runtime::tui::{DashboardCommand, map_key};
#[test]
fn maps_tab() {
    assert_eq!(map_key(KeyCode::Tab), DashboardCommand::NextTab);
}
