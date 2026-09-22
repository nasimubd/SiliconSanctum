use sanctum_runtime::speculative::{Lifecycle, SpeculativeError};
#[test]
fn rejects_illegal() {
    let mut state = Lifecycle::Idle;
    assert_eq!(
        state.transition(Lifecycle::Generating),
        Err(SpeculativeError::IllegalTransition)
    );
}
