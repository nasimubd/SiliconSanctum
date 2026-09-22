use sanctum_runtime::speculative::{ContextCapacity, ContextPosition};
#[test]
fn accepts_capacity_boundary() {
    let c = ContextCapacity::new(32768).unwrap();
    assert_eq!(ContextPosition::new(32768, c).unwrap().get(), 32768);
}
