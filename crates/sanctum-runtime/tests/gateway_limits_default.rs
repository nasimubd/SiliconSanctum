use sanctum_runtime::gateway::DetectionLimits;
#[test]
fn provides_detection_limits() {
    let v = DetectionLimits::default();
    assert_eq!(v.max_prompt_bytes, 65536);
}
