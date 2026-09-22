use sanctum_runtime::speculative::{ContextCapacity, SessionState, SpeculativeError};
#[test]
fn preserves_state_on_exhaustion() {
    let c = ContextCapacity::new(1).unwrap();
    let mut s = SessionState::new(c);
    s.advance(1, c).unwrap();
    assert_eq!(s.advance(1, c), Err(SpeculativeError::ContextExhausted));
    assert_eq!(s.position.get(), 1);
}
