use sanctum_runtime::router::*;
#[test]
fn bounds_event_history() {
    let mut v = RouterEventLog::new(1).unwrap();
    v.push(RouterEvent::FanoutStarted);
    v.push(RouterEvent::FanoutStarted);
    assert_eq!(v.entries().len(), 1);
}
