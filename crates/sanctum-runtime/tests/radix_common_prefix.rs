use sanctum_runtime::radix::common_prefix_len;
#[test]
fn calculates_common_prefix() {
    assert_eq!(common_prefix_len(&[1, 2, 3], &[1, 2, 4]), 2);
}
