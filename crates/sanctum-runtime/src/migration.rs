//! Thunderbolt storage migration and rollback.
#![allow(clippy::missing_errors_doc)]

#[derive(Debug,Clone,PartialEq,Eq)]
pub enum MigrationError{InvalidInput(&'static str),ToolFailure(String),Io(String)}
impl std::fmt::Display for MigrationError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{write!(f,"{self:?}")}}
impl std::error::Error for MigrationError{}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct LinkWidth(u8);
impl LinkWidth{pub fn new(lanes:u8)->Result<Self,MigrationError>{if lanes==0||lanes>16{return Err(MigrationError::InvalidInput("link width"));}Ok(Self(lanes))}pub const fn lanes(self)->u8{self.0}}
impl LinkWidth{pub fn parse(value:&str)->Result<Self,MigrationError>{let lanes=value.trim().trim_start_matches('x').parse::<u8>().map_err(|_|MigrationError::InvalidInput("link width"))?;Self::new(lanes)}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum LinkSpeed{Gt8,Gt16}
impl LinkSpeed{pub const fn gt_per_second(self)->u8{match self{Self::Gt8=>8,Self::Gt16=>16}}}
impl LinkSpeed{pub fn parse(value:&str)->Result<Self,MigrationError>{match value.trim(){ "8.0 GT/s"=>Ok(Self::Gt8),"16.0 GT/s"=>Ok(Self::Gt16),_=>Err(MigrationError::InvalidInput("link speed"))}}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum PortStatus{Connected,Disconnected}
impl PortStatus{pub fn parse(value:&str)->Self{let value=value.trim().to_ascii_lowercase();if value.contains("connected")&&!value.contains("no device"){Self::Connected}else{Self::Disconnected}}}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct BusInspection{pub width:LinkWidth,pub speed:LinkSpeed}
impl BusInspection{pub const fn qualifies(self)->bool{self.width.lanes()==4}}
// Migration extensions.
