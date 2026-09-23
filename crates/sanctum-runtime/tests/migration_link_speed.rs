use sanctum_runtime::migration::LinkSpeed;
#[test]
fn accepts_supported_speeds() {
    assert_eq!(LinkSpeed::parse("8.0 GT/s").unwrap(), LinkSpeed::Gt8);
    assert_eq!(LinkSpeed::parse("16.0 GT/s").unwrap(), LinkSpeed::Gt16);
}
