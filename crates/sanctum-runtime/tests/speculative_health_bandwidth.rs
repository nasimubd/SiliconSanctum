use sanctum_runtime::speculative::{BandwidthObservation, DecoderHealth, assess_health};
#[test]
fn detects_saturation() {
    assert_eq!(
        assess_health(0.9, BandwidthObservation::new(0.95).unwrap()),
        DecoderHealth::BandwidthSaturated
    );
}
