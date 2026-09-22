//! Interactive runtime telemetry dashboard.

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum DashboardError{ZeroRefreshRate,ZeroDimension,InvalidPercentage,InvalidMetric,EmptyProfile,Terminal(String)}
impl std::fmt::Display for DashboardError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for DashboardError{}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct RefreshRate(std::time::Duration);
impl RefreshRate{pub fn new(value:std::time::Duration)->Result<Self,DashboardError>{if value.is_zero(){return Err(DashboardError::ZeroRefreshRate);}Ok(Self(value))}pub const fn duration(self)->std::time::Duration{self.0}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct DashboardSize{pub width:u16,pub height:u16}
impl DashboardSize{pub fn new(width:u16,height:u16)->Result<Self,DashboardError>{if width==0||height==0{return Err(DashboardError::ZeroDimension);}Ok(Self{width,height})}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct TokenRate(f64);
impl TokenRate{pub fn new(value:f64)->Result<Self,DashboardError>{if !value.is_finite()||value<0.0{return Err(DashboardError::InvalidMetric);}Ok(Self(value))}pub const fn get(self)->f64{self.0}}
