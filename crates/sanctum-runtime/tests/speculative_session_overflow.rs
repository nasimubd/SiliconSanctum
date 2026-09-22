use sanctum_runtime::speculative::{ContextCapacity, SessionState, SpeculativeError};
#[test]
fn prevents_overflow() {
    let c = ContextCapacity::new(2).unwrap();
    let mut s = SessionState::new(c);
    assert_eq!(s.advance(3, c), Err(SpeculativeError::ContextExhausted));
}
