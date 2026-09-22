use crossterm::event::KeyCode;
use sanctum_runtime::tui::{map_key,DashboardCommand};
#[test] fn maps_refresh(){assert_eq!(map_key(KeyCode::Char('r')),DashboardCommand::Refresh);}
