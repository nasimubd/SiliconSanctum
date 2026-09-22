use sanctum_runtime::speculative::{
    AdaptationPolicy, BandwidthObservation, ContextCapacity, ContextPosition, DecoderConfig,
    DraftModel, ParameterCount, SpeculativeHarness, TargetModel,
};
#[test]
fn adapts_near_limit() {
    let capacity = ContextCapacity::new(100).unwrap();
    let h = SpeculativeHarness {
        config: DecoderConfig {
            draft: DraftModel::new("d", ParameterCount::new(1).unwrap()).unwrap(),
            target: TargetModel::new("t", ParameterCount::new(7_000_000_000).unwrap()).unwrap(),
            capacity,
        },
        policy: AdaptationPolicy::default(),
    };
    assert_eq!(
        h.width(
            ContextPosition::new(90, capacity).unwrap(),
            BandwidthObservation::new(0.5).unwrap()
        )
        .get(),
        1
    );
}
