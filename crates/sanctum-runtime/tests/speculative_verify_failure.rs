use sanctum_runtime::speculative::{
    DecoderBackend, DraftProposal, SpeculativeError, VerifyWidth, run_cycle,
};
struct B;
impl DecoderBackend for B {
    fn propose(&mut self, _: VerifyWidth) -> Result<DraftProposal, SpeculativeError> {
        DraftProposal::new(vec![1])
    }
    fn verify(&mut self, _: &DraftProposal) -> Result<Vec<u32>, SpeculativeError> {
        Err(SpeculativeError::Backend("target".into()))
    }
}
#[test]
fn propagates() {
    assert_eq!(
        run_cycle(&mut B, VerifyWidth::new(1).unwrap()),
        Err(SpeculativeError::Backend("target".into()))
    );
}
