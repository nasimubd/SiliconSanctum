use sanctum_runtime::plugin::{PluginEndpoint, PluginProtocol};
#[test]
fn formats_grpc_endpoint() {
    assert_eq!(
        PluginEndpoint::new(PluginProtocol::Grpc, "localhost:90")
            .unwrap()
            .uri(),
        "grpc://localhost:90"
    );
}
