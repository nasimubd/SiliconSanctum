use crossterm::event::KeyCode;
use sanctum_runtime::tui::{map_key,DashboardCommand};
#[test] fn maps_pause(){assert_eq!(map_key(KeyCode::Char(' ')),DashboardCommand::TogglePause);}
