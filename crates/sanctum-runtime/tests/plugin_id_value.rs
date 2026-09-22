use sanctum_runtime::plugin::PluginId;
#[test]
fn preserves_plugin_id() {
    assert_eq!(PluginId::new("tools").unwrap().as_str(), "tools");
}
