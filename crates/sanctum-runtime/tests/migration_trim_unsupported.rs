use sanctum_runtime::migration::{parse_trim_log,TrimStatus};
#[test]fn rejects_unsupported(){assert_eq!(parse_trim_log("kernel: spaceman: trim unsupported"),TrimStatus::Unsupported);}
