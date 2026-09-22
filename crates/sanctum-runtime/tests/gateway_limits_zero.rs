use sanctum_runtime::gateway::DetectionLimits;
#[test]
fn rejects_zero_detection_limits() {
    assert!(DetectionLimits::new(0, 1).is_err());
}
