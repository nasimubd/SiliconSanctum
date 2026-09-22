use sanctum_runtime::registry::ContextLadder;
#[test]
fn selects_context_floor() {
    let value = ContextLadder::new(vec![2048, 4096, 8192]).unwrap();
    assert_eq!(value.floor(6000), 4096);
}
