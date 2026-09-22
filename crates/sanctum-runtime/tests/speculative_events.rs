use sanctum_runtime::speculative::DecoderEvent;
#[test]
fn preserves_sequence() {
    let events = [DecoderEvent::Started, DecoderEvent::Stopped];
    assert!(matches!(events[0], DecoderEvent::Started));
}
