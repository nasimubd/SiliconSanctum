use sanctum_runtime::router::*;
#[test]
fn routes_high_boundary_to_speculation() {
    assert_eq!(
        RouterPolicy::default().select(0.95).destination,
        RouteDestination::SpeculativePipeline
    );
}
