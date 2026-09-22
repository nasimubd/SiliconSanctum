use sanctum_runtime::tui::{DashboardError,DashboardSize};
#[test]fn rejects_zero(){assert_eq!(DashboardSize::new(0,24),Err(DashboardError::ZeroDimension));}
