use sanctum_runtime::speculative::{DecoderEvent,EventLog};
#[test]fn evicts_oldest(){let mut log=EventLog::new(1).unwrap();log.push(DecoderEvent::Started);log.push(DecoderEvent::Stopped);assert_eq!(log.events().front(),Some(&DecoderEvent::Stopped));}
