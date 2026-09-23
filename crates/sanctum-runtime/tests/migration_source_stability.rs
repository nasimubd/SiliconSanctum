use sanctum_runtime::migration::{ManifestEntry, MigrationManifest, ensure_source_stable};

#[test]
fn detects_changed_source_entries() {
    let before = MigrationManifest { entries: Default::default() };
    let mut after = before.clone();
    assert!(ensure_source_stable(&before, &after).is_ok());
    after.entries.insert("new".into(), ManifestEntry::Directory);
    assert!(ensure_source_stable(&before, &after).is_err());
}
