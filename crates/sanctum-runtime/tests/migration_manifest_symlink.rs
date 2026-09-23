use sanctum_runtime::migration::{ManifestEntry, MigrationManifest};
#[cfg(unix)]
#[test]
fn records_symlink() {
    use std::os::unix::fs::symlink;
    let root = tempfile::tempdir().unwrap();
    symlink("/outside", root.path().join("shortcut")).unwrap();
    let manifest = MigrationManifest::scan(root.path()).unwrap();
    assert!(matches!(
        manifest.entries.get(std::path::Path::new("shortcut")),
        Some(ManifestEntry::Symlink(_))
    ));
}
