use sanctum_runtime::plugin::{PluginEndpoint, PluginProtocol};
#[test]
fn formats_http_endpoint() {
    assert_eq!(
        PluginEndpoint::new(PluginProtocol::Http, "localhost:80")
            .unwrap()
            .uri(),
        "http://localhost:80"
    );
}
