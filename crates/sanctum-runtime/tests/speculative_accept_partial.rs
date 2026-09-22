use sanctum_runtime::speculative::{DraftProposal, compare_proposal};
#[test]
fn accepts_partial() {
    let r = compare_proposal(&DraftProposal::new(vec![1, 2]).unwrap(), &[1, 9]);
    assert_eq!((r.accepted, r.fallback_token), (1, Some(9)));
}
