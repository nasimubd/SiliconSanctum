use sanctum_runtime::arbiter::{ArbiterDecision, ArbiterEvent, EventLog};
#[test]
fn evicts_oldest_event() {
    let mut log = EventLog::new(1).unwrap();
    log.push(ArbiterEvent::Decision(ArbiterDecision::hold()));
    log.push(ArbiterEvent::Decision(ArbiterDecision::reload()));
    assert_eq!(log.entries().len(), 1);
}
