use sanctum_runtime::plugin::PluginRequest;
#[test]
fn rejects_empty_request_id() {
    assert!(PluginRequest::new("", "tool", vec![]).is_err());
}
