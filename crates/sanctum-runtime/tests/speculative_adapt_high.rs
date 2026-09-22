use sanctum_runtime::speculative::{AdaptationPolicy,VerifyWidth,VerifyWidthBounds};
#[test] fn selects_minimum(){let p=AdaptationPolicy{widths:VerifyWidthBounds::new(VerifyWidth::new(2).unwrap(),VerifyWidth::new(8).unwrap()).unwrap(),medium_pressure:0.5,high_pressure:0.8};assert_eq!(p.select(0.9).get(),2);}
