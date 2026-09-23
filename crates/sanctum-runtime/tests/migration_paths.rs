use sanctum_runtime::migration::MigrationPaths;
use std::path::PathBuf;
#[test]fn rejects_nested_roots(){assert!(MigrationPaths::new(PathBuf::from("/Volumes/Origin/data"),PathBuf::from("/Volumes/Origin/data/target")).is_err());}
