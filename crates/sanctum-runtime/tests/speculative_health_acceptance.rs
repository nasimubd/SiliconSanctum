use sanctum_runtime::speculative::{BandwidthObservation, DecoderHealth, assess_health};
#[test]
fn detects_low_acceptance() {
    assert_eq!(
        assess_health(0.2, BandwidthObservation::new(0.5).unwrap()),
        DecoderHealth::LowAcceptance
    );
}
