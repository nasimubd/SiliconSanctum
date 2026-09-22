use sanctum_runtime::speculative::{CycleOutcome,DraftProposal,Verification};
#[test] fn counts_cycle(){let c=CycleOutcome{proposal:DraftProposal::new(vec![1]).unwrap(),verification:Verification{accepted:0,fallback_token:Some(2)}};assert_eq!(c.emitted_tokens(),1);}
