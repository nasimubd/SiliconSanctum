use sanctum_runtime::speculative::{run_cycle,DecoderBackend,DraftProposal,SpeculativeError,VerifyWidth};
struct B;impl DecoderBackend for B{fn propose(&mut self,_:VerifyWidth)->Result<DraftProposal,SpeculativeError>{DraftProposal::new(vec![1])}fn verify(&mut self,_:&DraftProposal)->Result<Vec<u32>,SpeculativeError>{Ok(vec![9])}}
#[test]fn emits_fallback(){assert_eq!(run_cycle(&mut B,VerifyWidth::new(1).unwrap()).unwrap().verification.fallback_token,Some(9));}
