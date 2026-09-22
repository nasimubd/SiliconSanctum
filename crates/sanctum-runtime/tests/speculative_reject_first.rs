use sanctum_runtime::speculative::{DraftProposal, compare_proposal};
#[test]
fn rejects_first() {
    let r = compare_proposal(&DraftProposal::new(vec![1]).unwrap(), &[9]);
    assert_eq!((r.accepted, r.fallback_token), (0, Some(9)));
}
