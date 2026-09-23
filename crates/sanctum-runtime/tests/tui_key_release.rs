use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use sanctum_runtime::tui::{DashboardCommand, map_key_event};
#[test]
fn ignores_key_release() {
    let mut event = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
    event.kind = KeyEventKind::Release;
    assert_eq!(map_key_event(event), DashboardCommand::Ignore);
}
