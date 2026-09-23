use crossterm::event::KeyCode;
use sanctum_runtime::tui::{DashboardCommand, map_key};
#[test]
fn ignores_unsupported() {
    assert_eq!(map_key(KeyCode::F(12)), DashboardCommand::Ignore);
}
