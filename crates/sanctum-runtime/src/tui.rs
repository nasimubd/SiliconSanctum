//! Interactive runtime telemetry dashboard.
use std::collections::VecDeque;

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
impl MemoryTelemetry{pub fn new(wired:MemoryBytes,os_cache:MemoryBytes,total:MemoryBytes)->Result<Self,DashboardError>{if total.0==0||wired.0>total.0||os_cache.0>total.0{return Err(DashboardError::InvalidMetric);}Ok(Self{wired,os_cache,total})}}
impl MemoryTelemetry{pub fn severity(self)->MetricSeverity{severity(self.wired.ratio(self.total))}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct Utilization(f32);
impl Utilization{pub fn new(value:f32)->Result<Self,DashboardError>{if !value.is_finite()||!(0.0..=1.0).contains(&value){return Err(DashboardError::InvalidPercentage);}Ok(Self(value))}pub const fn get(self)->f32{self.0}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct CpuTelemetry{pub performance:Utilization,pub efficiency:Utilization}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct KvResidency{pub occupied:u16,pub capacity:u16}
impl KvResidency{pub fn new(occupied:u16,capacity:u16)->Result<Self,DashboardError>{if capacity==0||occupied>capacity{return Err(DashboardError::InvalidMetric);}Ok(Self{occupied,capacity})}pub fn ratio(self)->f32{if self.capacity==0{0.0}else{f32::from(self.occupied)/f32::from(self.capacity)}}}
impl KvResidency{pub fn severity(self)->MetricSeverity{severity(f64::from(self.ratio()))}}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ActiveProfile(String);
impl ActiveProfile{pub fn new(value:impl Into<String>)->Result<Self,DashboardError>{let value=value.into();if value.trim().is_empty(){return Err(DashboardError::EmptyProfile);}Ok(Self(value))}pub fn as_str(&self)->&str{&self.0}}
#[derive(Debug,Clone,PartialEq)]
pub struct DashboardSnapshot{pub token_rate:TokenRate,pub memory:MemoryTelemetry,pub cpu:CpuTelemetry,pub kv:KvResidency,pub profile:ActiveProfile}
#[derive(Debug,Clone,PartialEq)]
pub struct MetricHistory{capacity:usize,token_rates:VecDeque<f64>,wired_ratios:VecDeque<f64>,performance_ratios:VecDeque<f32>}
impl MetricHistory{pub fn new(capacity:usize)->Result<Self,DashboardError>{if capacity==0{return Err(DashboardError::InvalidMetric);}Ok(Self{capacity,token_rates:VecDeque::new(),wired_ratios:VecDeque::new(),performance_ratios:VecDeque::new()})}}
impl DashboardSnapshot{pub fn new(token_rate:TokenRate,memory:MemoryTelemetry,cpu:CpuTelemetry,kv:KvResidency,profile:ActiveProfile)->Self{Self{token_rate,memory,cpu,kv,profile}}}
pub trait TelemetrySource{fn snapshot(&mut self)->Result<DashboardSnapshot,DashboardError>;}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum DashboardTab{Overview,Memory,Compute}
impl DashboardTab{pub const fn next(self)->Self{match self{Self::Overview=>Self::Memory,Self::Memory=>Self::Compute,Self::Compute=>Self::Overview}}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum DashboardFocus{Metrics,History,Help}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum MetricSeverity{Normal,Warning,Critical}
pub fn severity(ratio:f64)->MetricSeverity{if ratio>=0.9{MetricSeverity::Critical}else if ratio>=0.75{MetricSeverity::Warning}else{MetricSeverity::Normal}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum DashboardCommand{Quit,TogglePause,NextTab,Refresh,Ignore}
pub fn map_key(key:crossterm::event::KeyCode)->DashboardCommand{use crossterm::event::KeyCode;match key{KeyCode::Char('q')|KeyCode::Esc=>DashboardCommand::Quit,KeyCode::Char(' ')=>DashboardCommand::TogglePause,KeyCode::Tab=>DashboardCommand::NextTab,KeyCode::Char('r')=>DashboardCommand::Refresh,_=>DashboardCommand::Ignore}}
#[derive(Debug,Clone,PartialEq)]
pub struct DashboardState{pub snapshot:DashboardSnapshot,pub tab:DashboardTab,pub paused:bool,pub quit:bool}
impl DashboardState{pub fn new(snapshot:DashboardSnapshot)->Self{Self{snapshot,tab:DashboardTab::Overview,paused:false,quit:false}}}
impl DashboardState{pub fn apply(&mut self,command:DashboardCommand){match command{DashboardCommand::Quit=>self.quit=true,DashboardCommand::TogglePause=>self.paused=!self.paused,DashboardCommand::NextTab=>self.tab=self.tab.next(),DashboardCommand::Refresh|DashboardCommand::Ignore=>{}}}}
impl DashboardState{pub fn refresh<S:TelemetrySource>(&mut self,source:&mut S)->Result<(),DashboardError>{if !self.paused{self.snapshot=source.snapshot()?;}Ok(())}}
