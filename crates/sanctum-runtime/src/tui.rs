//! Interactive runtime telemetry dashboard.

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum DashboardError{ZeroRefreshRate,ZeroDimension,InvalidPercentage,InvalidMetric,EmptyProfile,Terminal(String)}
