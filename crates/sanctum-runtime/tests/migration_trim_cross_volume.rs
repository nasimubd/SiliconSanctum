use sanctum_runtime::migration::{TargetVolumeIdentity, TrimStatus, parse_trim_log_for_volume};
#[test]
fn ignores_other_volume() {
    let identity = TargetVolumeIdentity {
        volume_uuid: "98927CDF-4342-412B-AB62-231E18833833".into(),
        device_identifier: "disk5s1".into(),
    };
    assert_eq!(
        parse_trim_log_for_volume("kernel: spaceman disk8s1 trim completed", &identity),
        TrimStatus::Absent
    );
}
