use sanctum_runtime::speculative::{ParameterCount, SpeculativeError};
#[test]
fn rejects_zero_parameters() { assert_eq!(ParameterCount::new(0), Err(SpeculativeError::ZeroValue("parameter count"))); }
