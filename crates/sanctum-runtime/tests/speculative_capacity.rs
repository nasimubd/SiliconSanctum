use sanctum_runtime::speculative::{ContextCapacity,SpeculativeError};
#[test]
fn rejects_zero_capacity(){assert_eq!(ContextCapacity::new(0),Err(SpeculativeError::ZeroValue("context capacity")));}
