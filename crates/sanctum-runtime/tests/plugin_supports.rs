use sanctum_runtime::plugin::*;
#[test]
fn reports_supported_capability() {
    let mut value = PluginManifest::new(
        PluginId::new("p").unwrap(),
        PluginEndpoint::new(PluginProtocol::Http, "host").unwrap(),
    );
    value
        .add_capability(Capability::new("tool").unwrap())
        .unwrap();
    assert!(value.supports("tool"));
}
