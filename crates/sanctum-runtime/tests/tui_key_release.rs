use crossterm::event::{KeyCode,KeyEvent,KeyEventKind,KeyModifiers};
use sanctum_runtime::tui::{map_key_event,DashboardCommand};
#[test]fn ignores_key_release(){let mut event=KeyEvent::new(KeyCode::Char('q'),KeyModifiers::NONE);event.kind=KeyEventKind::Release;assert_eq!(map_key_event(event),DashboardCommand::Ignore);}
