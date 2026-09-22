use sanctum_runtime::router::*;
fn targets() -> RouteTargets {
    RouteTargets {
        deterministic: DeterministicTarget::new("tool").unwrap(),
        speculative: ModelPair::new("d", "t").unwrap(),
        heavy: HeavyModelTarget::new("h").unwrap(),
    }
}
#[test]
fn builds_tool_plan() {
    assert!(matches!(
        targets().plan(RouteDecision::deterministic(1.0)),
        RoutePlan::Deterministic(_)
    ));
}
