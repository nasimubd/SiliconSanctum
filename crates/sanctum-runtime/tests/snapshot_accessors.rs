use sanctum_runtime::arbiter::MemorySnapshot;
#[test]
fn exposes_snapshot_values() {
    let value = MemorySnapshot::new(1, 2, 3);
    assert_eq!(
        (
            value.wired_bytes(),
            value.available_bytes(),
            value.swap_used_bytes()
        ),
        (1, 2, 3)
    );
}
