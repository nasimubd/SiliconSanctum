use sanctum_runtime::migration::write_ai_root;
#[test]fn writes_absolute_root(){let root=tempfile::tempdir().unwrap();let pointer=root.path().join("ai-root");write_ai_root(&pointer,std::path::Path::new("/Volumes/A/data")).unwrap();assert_eq!(std::fs::read_to_string(pointer).unwrap(),"/Volumes/A/data\n");}
