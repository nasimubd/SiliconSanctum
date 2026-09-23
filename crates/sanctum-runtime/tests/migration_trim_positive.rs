use sanctum_runtime::migration::{TrimStatus, parse_trim_log};
#[test]
fn finds_trim_evidence() {
    assert_eq!(
        parse_trim_log("kernel: spaceman: trim completed"),
        TrimStatus::Observed
    );
}
