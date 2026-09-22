use sanctum_runtime::plugin::PluginId;
#[test]
fn rejects_empty_plugin_id() {
    assert!(PluginId::new(" ").is_err());
}
