use sanctum_runtime::speculative::{context_pressure,ContextCapacity,ContextPosition};
#[test]
fn reports_full_pressure(){let c=ContextCapacity::new(100).unwrap();assert_eq!(context_pressure(ContextPosition::new(100,c).unwrap(),c),1.0);}
