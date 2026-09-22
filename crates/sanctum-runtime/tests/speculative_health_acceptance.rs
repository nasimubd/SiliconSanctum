use sanctum_runtime::speculative::{assess_health,BandwidthObservation,DecoderHealth};
#[test]fn detects_low_acceptance(){assert_eq!(assess_health(0.2,BandwidthObservation::new(0.5).unwrap()),DecoderHealth::LowAcceptance);}
