use crossterm::event::KeyCode;
use sanctum_runtime::tui::{DashboardCommand, map_key};
#[test]
fn maps_refresh() {
    assert_eq!(map_key(KeyCode::Char('r')), DashboardCommand::Refresh);
}
