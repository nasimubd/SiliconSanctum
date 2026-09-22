use sanctum_runtime::prefill::{PrefillEvent, PrefillEventLog, PrefillStep};

#[test]
fn bounds_prefill_event_history() {
    let mut log = PrefillEventLog::new(1).unwrap();
    log.push(PrefillEvent::Cancelled);
    log.push(PrefillEvent::Planned {
        chunks: 2,
        step: PrefillStep::Tokens512,
    });
    assert_eq!(log.events().len(), 1);
    assert!(matches!(
        log.events().front(),
        Some(PrefillEvent::Planned { .. })
    ));
}
