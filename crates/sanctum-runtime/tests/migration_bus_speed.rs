use sanctum_runtime::migration::LinkSpeed;
#[test]
fn rejects_port_capability() {
    assert!(LinkSpeed::parse("Up to 40 Gb/s").is_err());
}
