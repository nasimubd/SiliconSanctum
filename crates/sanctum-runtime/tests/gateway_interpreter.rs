use sanctum_runtime::gateway::*;
#[test]
fn maps_counting_interpreter() {
    assert_eq!(
        BypassReason::Counting.interpreter(),
        Some(InterpreterKind::Counting)
    );
    assert_eq!(BypassReason::Temporal.interpreter(), None);
}
