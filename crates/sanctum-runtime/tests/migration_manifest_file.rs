use sanctum_runtime::migration::{ManifestEntry,MigrationManifest};
#[test]fn scans_file(){let root=tempfile::tempdir().unwrap();std::fs::write(root.path().join("model.bin"),b"weights").unwrap();let manifest=MigrationManifest::scan(root.path()).unwrap();assert!(matches!(manifest.entries.get(std::path::Path::new("model.bin")),Some(ManifestEntry::File{..})));}
