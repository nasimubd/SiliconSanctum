//! Adaptive speculative-decoding primitives.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpeculativeError {
    ZeroValue(&'static str),
    DraftModelTooLarge(u64),
    TargetModelTooSmall(u64),
    ContextPositionOutOfRange { position: u32, capacity: u32 },
    InvalidWidthBounds { minimum: u8, maximum: u8 },
    EmptyProposal,
    ContextExhausted,
    Cancelled,
    Backend(String),
    IllegalTransition,
}

impl fmt::Display for SpeculativeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{self:?}") }
}
impl std::error::Error for SpeculativeError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ParameterCount(u64);
impl ParameterCount {
    pub fn new(value: u64) -> Result<Self, SpeculativeError> { (value > 0).then_some(Self(value)).ok_or(SpeculativeError::ZeroValue("parameter count")) }
    pub const fn get(self) -> u64 { self.0 }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DraftModel { pub name: String, pub parameters: ParameterCount }
impl DraftModel {
 pub const MAX_PARAMETERS:u64=2_000_000_000;
 pub fn new(name:impl Into<String>,parameters:ParameterCount)->Result<Self,SpeculativeError>{
  if parameters.get()>=Self::MAX_PARAMETERS{return Err(SpeculativeError::DraftModelTooLarge(parameters.get()));}
  Ok(Self{name:name.into(),parameters})
 }
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct TargetModel{pub name:String,pub parameters:ParameterCount}
impl TargetModel{
 pub const MIN_PARAMETERS:u64=7_000_000_000;
 pub fn new(name:impl Into<String>,parameters:ParameterCount)->Result<Self,SpeculativeError>{
  if parameters.get()<Self::MIN_PARAMETERS{return Err(SpeculativeError::TargetModelTooSmall(parameters.get()));}
  Ok(Self{name:name.into(),parameters})
 }
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ContextCapacity(u32);
impl ContextCapacity{
 pub fn new(value:u32)->Result<Self,SpeculativeError>{(value>0).then_some(Self(value)).ok_or(SpeculativeError::ZeroValue("context capacity"))}
 pub const fn get(self)->u32{self.0}
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct ContextPosition(u32);
impl ContextPosition{
 pub fn new(value:u32,capacity:ContextCapacity)->Result<Self,SpeculativeError>{
  if value>capacity.get(){return Err(SpeculativeError::ContextPositionOutOfRange{position:value,capacity:capacity.get()});} Ok(Self(value))
 }
 pub const fn get(self)->u32{self.0}
}
#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct VerifyWidth(u8);
impl VerifyWidth{
 pub fn new(value:u8)->Result<Self,SpeculativeError>{(value>0).then_some(Self(value)).ok_or(SpeculativeError::ZeroValue("verify width"))}
 pub const fn get(self)->u8{self.0}
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct VerifyWidthBounds{pub minimum:VerifyWidth,pub maximum:VerifyWidth}
impl VerifyWidthBounds{
 pub fn new(minimum:VerifyWidth,maximum:VerifyWidth)->Result<Self,SpeculativeError>{
  if minimum>maximum{return Err(SpeculativeError::InvalidWidthBounds{minimum:minimum.get(),maximum:maximum.get()});} Ok(Self{minimum,maximum})
 }
}
// NEXT
