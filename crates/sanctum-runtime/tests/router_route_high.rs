use sanctum_runtime::router::*;
#[test]
fn routes_high_confidence_to_tool() {
    assert_eq!(
        RouterPolicy::default().select(0.96).destination,
        RouteDestination::DeterministicTool
    );
}
