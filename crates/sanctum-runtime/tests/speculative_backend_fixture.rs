use sanctum_runtime::speculative::{DecoderBackend, DraftProposal, SpeculativeError, VerifyWidth};
struct Backend;
impl DecoderBackend for Backend {
    fn propose(&mut self, _: VerifyWidth) -> Result<DraftProposal, SpeculativeError> {
        DraftProposal::new(vec![1])
    }
    fn verify(&mut self, _: &DraftProposal) -> Result<Vec<u32>, SpeculativeError> {
        Ok(vec![1])
    }
}
#[test]
fn fixture_proposes() {
    assert_eq!(
        Backend
            .propose(VerifyWidth::new(1).unwrap())
            .unwrap()
            .tokens(),
        &[1]
    );
}
