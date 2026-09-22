//! Adaptive speculative-decoding primitives.

use std::fmt;
use std::sync::{Arc,atomic::{AtomicBool,Ordering}};

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
pub fn context_pressure(position:ContextPosition,capacity:ContextCapacity)->f32{position.get() as f32/capacity.get() as f32}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct AdaptationPolicy{pub widths:VerifyWidthBounds,pub medium_pressure:f32,pub high_pressure:f32}
impl AdaptationPolicy{
 pub fn select(&self,pressure:f32)->VerifyWidth{
  if pressure>=self.high_pressure{self.widths.minimum}else if pressure>=self.medium_pressure{VerifyWidth::new((self.widths.maximum.get()/2).max(self.widths.minimum.get())).expect("bounded width")}else{self.widths.maximum}
 }
 pub fn select_with_bandwidth(&self,pressure:f32,bandwidth:BandwidthObservation)->VerifyWidth{if bandwidth.utilization()>=0.9{self.widths.minimum}else{self.select(pressure)}}
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct DraftProposal(Vec<u32>);
impl DraftProposal{pub fn new(tokens:Vec<u32>)->Result<Self,SpeculativeError>{(!tokens.is_empty()).then_some(Self(tokens)).ok_or(SpeculativeError::EmptyProposal)}pub fn tokens(&self)->&[u32]{&self.0}}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Verification{pub accepted:usize,pub fallback_token:Option<u32>}
impl Verification{pub fn emitted_tokens(&self)->usize{self.accepted+usize::from(self.fallback_token.is_some())}}
pub fn compare_proposal(proposal:&DraftProposal,target:&[u32])->Verification{let accepted=proposal.tokens().iter().zip(target).take_while(|(draft,target)|draft==target).count();Verification{accepted,fallback_token:target.get(accepted).copied()}}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct CycleOutcome{pub proposal:DraftProposal,pub verification:Verification}
impl CycleOutcome{pub fn emitted_tokens(&self)->usize{self.verification.emitted_tokens()}}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct DecoderConfig{pub draft:DraftModel,pub target:TargetModel,pub capacity:ContextCapacity}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct SessionState{pub position:ContextPosition,pub emitted:u64}
impl SessionState{pub fn new(capacity:ContextCapacity)->Self{Self{position:ContextPosition::new(0,capacity).expect("zero position"),emitted:0}}pub fn advance(&mut self,tokens:u32,capacity:ContextCapacity)->Result<(),SpeculativeError>{let next=self.position.get().checked_add(tokens).ok_or(SpeculativeError::ContextExhausted)?;self.position=ContextPosition::new(next,capacity).map_err(|_|SpeculativeError::ContextExhausted)?;self.emitted+=u64::from(tokens);Ok(())}}
#[derive(Debug,Clone,Copy,Default,PartialEq,Eq)]
pub struct AcceptanceMetrics{pub proposed:u64,pub accepted:u64}
impl AcceptanceMetrics{pub fn rate(self)->f64{if self.proposed==0{0.0}else{self.accepted as f64/self.proposed as f64}}pub fn record(&mut self,proposed:usize,accepted:usize){self.proposed+=proposed as u64;self.accepted+=accepted as u64;}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct BandwidthObservation(f32);
impl BandwidthObservation{pub fn new(value:f32)->Result<Self,SpeculativeError>{(value.is_finite()&&(0.0..=1.0).contains(&value)).then_some(Self(value)).ok_or(SpeculativeError::ZeroValue("bandwidth utilization range"))}pub const fn utilization(self)->f32{self.0}}
pub trait DecoderBackend{
 fn propose(&mut self,width:VerifyWidth)->Result<DraftProposal,SpeculativeError>;
 fn verify(&mut self,proposal:&DraftProposal)->Result<Vec<u32>,SpeculativeError>;
}
pub fn run_cycle<B:DecoderBackend>(backend:&mut B,width:VerifyWidth)->Result<CycleOutcome,SpeculativeError>{let proposal=backend.propose(width)?;let target=backend.verify(&proposal)?;let verification=compare_proposal(&proposal,&target);Ok(CycleOutcome{proposal,verification})}
pub fn generate<B:DecoderBackend>(backend:&mut B,state:&mut SessionState,capacity:ContextCapacity,width:VerifyWidth,requested:u32)->Result<Vec<CycleOutcome>,SpeculativeError>{let mut cycles=Vec::new();while state.emitted<u64::from(requested){let cycle=run_cycle(backend,width)?;let emitted=u32::try_from(cycle.emitted_tokens()).map_err(|_|SpeculativeError::ContextExhausted)?;if emitted==0{return Err(SpeculativeError::Backend("cycle emitted no tokens".into()));}state.advance(emitted,capacity)?;cycles.push(cycle);}Ok(cycles)}
#[derive(Debug,Clone,Default)]
pub struct CancellationToken(Arc<AtomicBool>);
impl CancellationToken{pub fn cancel(&self){self.0.store(true,Ordering::Release)}pub fn check(&self)->Result<(),SpeculativeError>{if self.0.load(Ordering::Acquire){Err(SpeculativeError::Cancelled)}else{Ok(())}}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct TimingSample{pub tokens:u64,pub elapsed_seconds:f64}
impl TimingSample{pub fn tokens_per_second(self)->f64{if self.elapsed_seconds<=0.0{0.0}else{self.tokens as f64/self.elapsed_seconds}}}
#[derive(Debug,Clone,Copy,PartialEq)]
pub struct TelemetrySnapshot{pub tokens_per_second:f64,pub acceptance_rate:f64,pub verify_width:u8,pub context_pressure:f32,pub bandwidth_utilization:f32}
#[derive(Debug,Clone,PartialEq,Eq)]
pub struct ModelProfile{pub draft:String,pub target:String}
impl ModelProfile{pub fn label(&self)->String{format!("{} -> {}",self.draft,self.target)}}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum DecoderEvent{Started,CycleCompleted{accepted:usize,proposed:usize},WidthChanged{from:u8,to:u8},Stopped}
// NEXT
