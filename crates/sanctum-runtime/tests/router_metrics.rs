use sanctum_runtime::router::*;
#[test]
fn records_route_metrics() {
    let mut v = RouteMetrics::default();
    v.record(RouteDestination::DeterministicTool);
    v.record(RouteDestination::SpeculativePipeline);
    v.record(RouteDestination::HeavyModel);
    assert_eq!((v.deterministic, v.speculative, v.heavy), (1, 1, 1));
}
