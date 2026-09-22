use sanctum_runtime::tui::{DashboardError,RefreshRate};
#[test]fn rejects_zero(){assert_eq!(RefreshRate::new(std::time::Duration::ZERO),Err(DashboardError::ZeroRefreshRate));}
