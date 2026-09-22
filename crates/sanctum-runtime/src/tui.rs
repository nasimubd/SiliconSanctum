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
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct MemoryBytes(pub u64);
impl MemoryBytes{pub fn ratio(self,total:Self)->f64{if total.0==0{0.0}else{self.0 as f64/total.0 as f64}}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct MemoryTelemetry{pub wired:MemoryBytes,pub os_cache:MemoryBytes,pub total:MemoryBytes}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Utilization(f32);
impl Utilization{pub fn new(value:f32)->Result<Self,DashboardError>{if !value.is_finite()||!(0.0..=1.0).contains(&value){return Err(DashboardError::InvalidPercentage);}Ok(Self(value))}pub const fn get(self)->f32{self.0}}
