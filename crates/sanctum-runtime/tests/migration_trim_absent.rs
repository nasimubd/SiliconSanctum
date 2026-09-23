use sanctum_runtime::migration::{TrimStatus, parse_trim_log};
#[test]
fn does_not_infer_trim() {
    assert_eq!(
        parse_trim_log("kernel: spaceman: checkpoint committed"),
        TrimStatus::Absent
    );
}
