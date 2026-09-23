use sanctum_runtime::migration::{parse_trim_log,TrimStatus};
#[test]fn finds_trim_evidence(){assert_eq!(parse_trim_log("kernel: spaceman: trim completed"),TrimStatus::Observed);}
