use sanctum_runtime::speculative::{DraftModel, ParameterCount, SpeculativeError};
#[test]
fn enforces_draft_ceiling() {
    let c = ParameterCount::new(2_000_000_000).unwrap();
    assert_eq!(
        DraftModel::new("d", c),
        Err(SpeculativeError::DraftModelTooLarge(c.get()))
    );
}
