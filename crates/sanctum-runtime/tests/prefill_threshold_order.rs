use sanctum_runtime::prefill::{PrefillError, TuningThresholds};

#[test]
fn rejects_inverted_tuning_thresholds() {
    assert_eq!(
        TuningThresholds::new(2048, 1024),
        Err(PrefillError::InvalidThresholds)
    );
}
