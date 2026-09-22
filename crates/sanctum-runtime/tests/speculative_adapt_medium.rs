use sanctum_runtime::speculative::{AdaptationPolicy,VerifyWidth,VerifyWidthBounds};
#[test] fn halves_maximum(){let p=AdaptationPolicy{widths:VerifyWidthBounds::new(VerifyWidth::new(1).unwrap(),VerifyWidth::new(8).unwrap()).unwrap(),medium_pressure:0.5,high_pressure:0.8};assert_eq!(p.select(0.6).get(),4);}
