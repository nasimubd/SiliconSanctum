use sanctum_runtime::router::*;
fn targets() -> RouteTargets {
    RouteTargets {
        deterministic: DeterministicTarget::new("tool").unwrap(),
        speculative: ModelPair::new("d", "t").unwrap(),
        heavy: HeavyModelTarget::new("h").unwrap(),
    }
}
#[test]
fn builds_speculative_plan() {
    assert!(matches!(
        targets().plan(RouteDecision::speculative(0.8)),
        RoutePlan::Speculative(_)
    ));
}
