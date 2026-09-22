use sanctum_runtime::radix::CacheCapacity;
#[test]
fn rejects_zero_cache_capacity() {
    assert!(CacheCapacity::new(0, 1).is_err());
}
