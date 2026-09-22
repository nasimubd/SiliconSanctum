use sanctum_runtime::plugin::{PluginResponse, PluginStatus};
#[test]
fn constructs_success_response() {
    assert_eq!(
        PluginResponse::success("1", vec![1]).status,
        PluginStatus::Ok
    );
}
