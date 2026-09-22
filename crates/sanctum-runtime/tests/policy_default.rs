use sanctum_runtime::arbiter::{ArbiterPolicy, WIRED_LIMIT_BYTES};
#[test]
fn defaults_to_wired_ceiling() {
    assert_eq!(
        ArbiterPolicy::default().wired_limit_bytes,
        WIRED_LIMIT_BYTES
    );
}
