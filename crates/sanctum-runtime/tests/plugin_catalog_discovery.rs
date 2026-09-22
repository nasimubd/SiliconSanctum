use sanctum_runtime::plugin::*;
#[test]
fn discovers_capable_plugins() {
    let mut manifest = PluginManifest::new(
        PluginId::new("p").unwrap(),
        PluginEndpoint::new(PluginProtocol::Grpc, "host").unwrap(),
    );
    manifest
        .add_capability(Capability::new("tool").unwrap())
        .unwrap();
    let mut catalog = PluginCatalog::default();
    catalog.register(manifest);
    assert_eq!(catalog.for_capability("tool").len(), 1);
}
