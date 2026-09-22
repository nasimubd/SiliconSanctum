use sanctum_runtime::speculative::AdaptationPolicy;
#[test]
fn defaults_for_bandwidth() {
    let p = AdaptationPolicy::default();
    assert_eq!((p.widths.minimum.get(), p.widths.maximum.get()), (1, 8));
}
