use sanctum_runtime::speculative::BandwidthObservation;
#[test]
fn rejects_excess_utilization() {
    assert!(BandwidthObservation::new(1.01).is_err());
}
