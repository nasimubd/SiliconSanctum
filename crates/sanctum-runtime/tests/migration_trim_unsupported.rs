use sanctum_runtime::migration::{TrimStatus, parse_trim_log};
#[test]
fn rejects_unsupported() {
    assert_eq!(
        parse_trim_log("kernel: spaceman: trim unsupported"),
        TrimStatus::Unsupported
    );
}
