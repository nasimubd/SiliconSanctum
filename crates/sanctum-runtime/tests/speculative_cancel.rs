use sanctum_runtime::speculative::{CancellationToken,SpeculativeError};
#[test]fn observes_cancel(){let token=CancellationToken::default();token.cancel();assert_eq!(token.check(),Err(SpeculativeError::Cancelled));}
