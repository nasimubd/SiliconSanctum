use sanctum_runtime::migration::MigrationStage;
#[test]fn rejects_skipped_stage(){let mut stage=MigrationStage::Prepared;assert!(stage.transition(MigrationStage::Activated).is_err());}
