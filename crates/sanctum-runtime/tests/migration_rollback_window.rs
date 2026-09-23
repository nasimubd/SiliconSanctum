use sanctum_runtime::migration::{
    CutoverRecord, MigrationPaths, MigrationStage, ROLLBACK_WINDOW_SECONDS,
};
#[test]
fn expires_after_window() {
    let record = CutoverRecord {
        paths: MigrationPaths::new("/Volumes/A/data".into(), "/Volumes/B/data".into()).unwrap(),
        stage: MigrationStage::Activated,
        activated_at: 100,
    };
    assert!(record.rollback_active(100 + ROLLBACK_WINDOW_SECONDS));
    assert!(!record.rollback_active(101 + ROLLBACK_WINDOW_SECONDS));
}
