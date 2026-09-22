use sanctum_runtime::arbiter::DaemonConfig;
#[test]
fn rejects_zero_sampling_interval() {
    assert!(DaemonConfig::new(std::time::Duration::ZERO, 1).is_err());
}
