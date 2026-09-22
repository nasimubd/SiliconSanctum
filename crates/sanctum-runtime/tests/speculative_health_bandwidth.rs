use sanctum_runtime::speculative::{assess_health,BandwidthObservation,DecoderHealth};
#[test]fn detects_saturation(){assert_eq!(assess_health(0.9,BandwidthObservation::new(0.95).unwrap()),DecoderHealth::BandwidthSaturated);}
