use crossterm::event::KeyCode;
use sanctum_runtime::tui::{map_key,DashboardCommand};
#[test] fn ignores_unsupported(){assert_eq!(map_key(KeyCode::F(12)),DashboardCommand::Ignore);}
