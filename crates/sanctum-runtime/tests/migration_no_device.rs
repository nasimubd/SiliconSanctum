use sanctum_runtime::migration::BusInspection;
#[test]
fn rejects_empty_port() {
    assert!(
        BusInspection::parse(
            "Thunderbolt/USB4:\n  Port:\n    Status: No device connected\n    Speed: Up to 40 Gb/s"
        )
        .is_err()
    );
}
