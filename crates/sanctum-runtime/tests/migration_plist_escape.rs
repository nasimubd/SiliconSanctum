use sanctum_runtime::migration::escape_plist_xml;

#[test]
fn escapes_all_xml_metacharacters() {
    assert_eq!(escape_plist_xml("&<>\"'"), "&amp;&lt;&gt;&quot;&apos;");
}
