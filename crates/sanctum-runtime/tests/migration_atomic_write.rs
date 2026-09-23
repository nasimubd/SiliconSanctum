use sanctum_runtime::migration::write_atomic;
#[test]fn replaces_state_file(){let root=tempfile::tempdir().unwrap();let path=root.path().join("state");write_atomic(&path,b"before").unwrap();write_atomic(&path,b"after").unwrap();assert_eq!(std::fs::read(path).unwrap(),b"after");}
