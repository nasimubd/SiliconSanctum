use sanctum_runtime::migration::{ManifestDifference, MigrationManifest, compare_manifests};
#[test]
fn detects_changed_file() {
    let a = tempfile::tempdir().unwrap();
    let b = tempfile::tempdir().unwrap();
    std::fs::write(a.path().join("model"), b"a").unwrap();
    std::fs::write(b.path().join("model"), b"b").unwrap();
    let error = compare_manifests(
        &MigrationManifest::scan(a.path()).unwrap(),
        &MigrationManifest::scan(b.path()).unwrap(),
    )
    .unwrap_err();
    assert_eq!(error, ManifestDifference::Changed("model".into()));
}
