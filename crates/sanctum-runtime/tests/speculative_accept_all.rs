use sanctum_runtime::speculative::{DraftProposal, compare_proposal};
#[test]
fn accepts_all() {
    let p = DraftProposal::new(vec![1, 2]).unwrap();
    assert_eq!(compare_proposal(&p, &[1, 2]).accepted, 2);
}
