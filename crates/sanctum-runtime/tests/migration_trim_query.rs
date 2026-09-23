use sanctum_runtime::migration::{trim_log_arguments,TRIM_LOG_PREDICATE};
#[test]fn queries_kernel_spaceman(){assert_eq!(trim_log_arguments()[0],"show");assert!(TRIM_LOG_PREDICATE.contains("spaceman"));}
