use sanctum_runtime::speculative::{
    DecoderBackend, DraftProposal, SpeculativeError, VerifyWidth, run_cycle,
};
struct B;
impl DecoderBackend for B {
    fn propose(&mut self, _: VerifyWidth) -> Result<DraftProposal, SpeculativeError> {
        Err(SpeculativeError::Backend("draft".into()))
    }
    fn verify(&mut self, _: &DraftProposal) -> Result<Vec<u32>, SpeculativeError> {
        unreachable!()
    }
}
#[test]
fn propagates() {
    assert_eq!(
        run_cycle(&mut B, VerifyWidth::new(1).unwrap()),
        Err(SpeculativeError::Backend("draft".into()))
    );
}
