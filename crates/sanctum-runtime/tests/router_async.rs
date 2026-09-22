use sanctum_runtime::router::*;
#[tokio::test]
async fn evaluates_axes_concurrently() {
    let e = FixedAxisEvaluator::new(0.9, std::time::Duration::ZERO).unwrap();
    let r = FanoutRouter::new(e.clone(), e.clone(), e, RouterPolicy::default());
    let v = r
        .evaluate(&RoutingPrompt::new("route").unwrap())
        .await
        .unwrap();
    assert_eq!(v.evaluations().len(), 3);
}
