use sanctum_runtime::router::*;
fn targets() -> RouteTargets {
    RouteTargets {
        deterministic: DeterministicTarget::new("tool").unwrap(),
        speculative: ModelPair::new("d", "t").unwrap(),
        heavy: HeavyModelTarget::new("h").unwrap(),
    }
}
#[test]
fn builds_heavy_plan() {
    assert!(matches!(
        targets().plan(RouteDecision::heavy(0.2)),
        RoutePlan::Heavy(_)
    ));
}
