use sanctum_runtime::migration::{TRIM_LOG_PREDICATE, trim_log_arguments};
#[test]
fn queries_kernel_spaceman() {
    assert_eq!(trim_log_arguments()[0], "show");
    assert!(TRIM_LOG_PREDICATE.contains("spaceman"));
}
