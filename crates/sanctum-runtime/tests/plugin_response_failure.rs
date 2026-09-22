use sanctum_runtime::plugin::{PluginResponse, PluginStatus};
#[test]
fn constructs_failure_response() {
    let value = PluginResponse::failure("1", PluginStatus::Unavailable);
    assert!(value.payload.is_empty());
}
