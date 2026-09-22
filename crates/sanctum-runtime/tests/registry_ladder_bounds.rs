use sanctum_runtime::registry::ContextLadder;
#[test]
fn exposes_context_bounds() {
    let value = ContextLadder::new(vec![2048, 4096]).unwrap();
    assert_eq!((value.smallest(), value.largest()), (2048, 4096));
}
