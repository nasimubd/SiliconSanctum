use sanctum_runtime::migration::{CutoverRecord,MigrationPaths,MigrationStage};
#[test]fn round_trips(){let record=CutoverRecord{paths:MigrationPaths::new("/Volumes/A/data".into(),"/Volumes/B/data".into()).unwrap(),stage:MigrationStage::Prepared,activated_at:0};assert_eq!(CutoverRecord::decode(&record.encode().unwrap()).unwrap(),record);}
