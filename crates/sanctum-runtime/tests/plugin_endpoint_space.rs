use sanctum_runtime::plugin::{PluginEndpoint, PluginProtocol};
#[test]
fn rejects_spaced_plugin_endpoint() {
    assert!(PluginEndpoint::new(PluginProtocol::Http, "host name").is_err());
}
