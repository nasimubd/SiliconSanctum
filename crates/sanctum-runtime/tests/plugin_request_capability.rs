use sanctum_runtime::plugin::PluginRequest;
#[test]
fn rejects_empty_request_capability() {
    assert!(PluginRequest::new("1", "", vec![]).is_err());
}
