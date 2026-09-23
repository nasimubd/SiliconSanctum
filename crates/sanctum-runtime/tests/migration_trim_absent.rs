use sanctum_runtime::migration::{parse_trim_log,TrimStatus};
#[test]fn does_not_infer_trim(){assert_eq!(parse_trim_log("kernel: spaceman: checkpoint committed"),TrimStatus::Absent);}
