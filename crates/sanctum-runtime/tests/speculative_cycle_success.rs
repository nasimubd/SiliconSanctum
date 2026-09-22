use sanctum_runtime::speculative::{
    DecoderBackend, DraftProposal, SpeculativeError, VerifyWidth, run_cycle,
};
struct B;
impl DecoderBackend for B {
    fn propose(&mut self, _: VerifyWidth) -> Result<DraftProposal, SpeculativeError> {
        DraftProposal::new(vec![1, 2])
    }
    fn verify(&mut self, _: &DraftProposal) -> Result<Vec<u32>, SpeculativeError> {
        Ok(vec![1, 2])
    }
}
#[test]
fn executes() {
    assert_eq!(
        run_cycle(&mut B, VerifyWidth::new(2).unwrap())
            .unwrap()
            .verification
            .accepted,
        2
    );
}
