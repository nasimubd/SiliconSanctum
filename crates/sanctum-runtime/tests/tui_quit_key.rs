use crossterm::event::KeyCode;
use sanctum_runtime::tui::{DashboardCommand, map_key};
#[test]
fn maps_quit() {
    assert_eq!(map_key(KeyCode::Char('q')), DashboardCommand::Quit);
    assert_eq!(map_key(KeyCode::Esc), DashboardCommand::Quit);
}
