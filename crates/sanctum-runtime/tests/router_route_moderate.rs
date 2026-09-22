use sanctum_runtime::router::*;
#[test]
fn routes_moderate_confidence_to_speculation() {
    assert_eq!(
        RouterPolicy::default().select(0.60).destination,
        RouteDestination::SpeculativePipeline
    );
}
