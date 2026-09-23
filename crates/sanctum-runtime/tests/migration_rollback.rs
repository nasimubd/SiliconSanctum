use sanctum_runtime::migration::{
    CutoverRecord, MigrationPaths, MigrationStage, rollback, save_record, write_ai_root,
};
#[test]
fn restores_origin() {
    let root = tempfile::tempdir().unwrap();
    let origin = root.path().join("origin");
    let target = root.path().join("target");
    std::fs::create_dir_all(&origin).unwrap();
    std::fs::create_dir_all(&target).unwrap();
    let record = CutoverRecord {
        paths: MigrationPaths::new(origin.clone(), target.clone()).unwrap(),
        stage: MigrationStage::Activated,
        activated_at: 1,
    };
    let state = root.path().join("state.json");
    let pointer = root.path().join("ai-root");
    save_record(&state, &record).unwrap();
    write_ai_root(&pointer, &target).unwrap();
    let result = rollback(&state, &pointer).unwrap();
    assert_eq!(result.stage, MigrationStage::RolledBack);
    assert_eq!(
        std::fs::read_to_string(pointer).unwrap(),
        format!("{}\n", origin.display())
    );
}
