use sanctum_runtime::speculative::{ContextCapacity, SessionState};
#[test]
fn starts_empty() {
    let s = SessionState::new(ContextCapacity::new(32).unwrap());
    assert_eq!((s.position.get(), s.emitted), (0, 0));
}
