use sanctum_runtime::migration::MigrationError;
#[test]
fn names_invalid_input() {
    assert!(
        MigrationError::InvalidInput("path")
            .to_string()
            .contains("path")
    );
}
