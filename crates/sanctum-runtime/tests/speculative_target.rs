use sanctum_runtime::speculative::{ParameterCount,SpeculativeError,TargetModel};
#[test]
fn enforces_target_floor(){let c=ParameterCount::new(6_999_999_999).unwrap();assert_eq!(TargetModel::new("t",c),Err(SpeculativeError::TargetModelTooSmall(c.get())));}
