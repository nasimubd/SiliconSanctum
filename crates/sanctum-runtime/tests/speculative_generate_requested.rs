use sanctum_runtime::speculative::{generate,ContextCapacity,DecoderBackend,DraftProposal,SessionState,SpeculativeError,VerifyWidth};
struct B;impl DecoderBackend for B{fn propose(&mut self,_:VerifyWidth)->Result<DraftProposal,SpeculativeError>{DraftProposal::new(vec![1])}fn verify(&mut self,_:&DraftProposal)->Result<Vec<u32>,SpeculativeError>{Ok(vec![1])}}
#[test]fn stops(){let c=ContextCapacity::new(8).unwrap();let mut s=SessionState::new(c);assert_eq!(generate(&mut B,&mut s,c,VerifyWidth::new(1).unwrap(),2).unwrap().len(),2);}
