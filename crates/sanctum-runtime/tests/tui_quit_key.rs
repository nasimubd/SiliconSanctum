use crossterm::event::KeyCode;
use sanctum_runtime::tui::{map_key,DashboardCommand};
#[test] fn maps_quit(){assert_eq!(map_key(KeyCode::Char('q')),DashboardCommand::Quit);assert_eq!(map_key(KeyCode::Esc),DashboardCommand::Quit);}
