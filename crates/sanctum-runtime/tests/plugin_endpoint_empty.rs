use sanctum_runtime::plugin::{PluginEndpoint, PluginProtocol};
#[test]
fn rejects_empty_plugin_endpoint() {
    assert!(PluginEndpoint::new(PluginProtocol::Http, "").is_err());
}
