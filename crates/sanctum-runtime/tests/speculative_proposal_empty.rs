use sanctum_runtime::speculative::{DraftProposal, SpeculativeError};
#[test]
fn rejects_empty() {
    assert_eq!(
        DraftProposal::new(vec![]),
        Err(SpeculativeError::EmptyProposal)
    );
}
