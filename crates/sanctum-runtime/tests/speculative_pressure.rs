use sanctum_runtime::speculative::{ContextCapacity, ContextPosition, context_pressure};
#[test]
fn reports_full_pressure() {
    let c = ContextCapacity::new(100).unwrap();
    assert!(
        (context_pressure(ContextPosition::new(100, c).unwrap(), c) - 1.0).abs() < f32::EPSILON
    );
}
