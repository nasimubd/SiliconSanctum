use sanctum_runtime::migration::MigrationManifest;
#[cfg(unix)]#[test]fn rejects_symlinked_root(){use std::os::unix::fs::symlink;let root=tempfile::tempdir().unwrap();let link=root.path().join("link");symlink(root.path(),&link).unwrap();assert!(MigrationManifest::scan(&link).is_err());}
