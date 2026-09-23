use crossterm::event::KeyCode;
use sanctum_runtime::tui::{DashboardCommand, map_key};
#[test]
fn maps_pause() {
    assert_eq!(map_key(KeyCode::Char(' ')), DashboardCommand::TogglePause);
}
