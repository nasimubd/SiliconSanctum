use sanctum_runtime::speculative::{ContextCapacity,DecoderConfig,DraftModel,ParameterCount,TargetModel};
#[test] fn preserves_capacity(){let c=DecoderConfig{draft:DraftModel::new("d",ParameterCount::new(1).unwrap()).unwrap(),target:TargetModel::new("t",ParameterCount::new(7_000_000_000).unwrap()).unwrap(),capacity:ContextCapacity::new(32).unwrap()};assert_eq!(c.capacity.get(),32);}
