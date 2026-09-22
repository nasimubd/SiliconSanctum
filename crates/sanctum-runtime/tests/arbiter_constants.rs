use sanctum_runtime::arbiter::{BYTES_PER_GIB, WIRED_LIMIT_BYTES};

#[test]
fn defines_binary_gibibyte() {
    assert_eq!(BYTES_PER_GIB, 1_073_741_824);
    assert_eq!(WIRED_LIMIT_BYTES, 10_905_190_400);
}
