use crossterm::event::KeyCode;
use sanctum_runtime::tui::{map_key,DashboardCommand};
#[test] fn maps_tab(){assert_eq!(map_key(KeyCode::Tab),DashboardCommand::NextTab);}
