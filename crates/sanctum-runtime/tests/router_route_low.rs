use sanctum_runtime::router::*;
#[test]
fn routes_low_confidence_to_heavy_model() {
    assert_eq!(
        RouterPolicy::default().select(0.59).destination,
        RouteDestination::HeavyModel
    );
}
