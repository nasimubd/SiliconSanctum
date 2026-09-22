use sanctum_runtime::speculative::{
    DecoderBackend, DraftProposal, SpeculativeError, VerifyWidth, run_cycle,
};
struct B;
impl DecoderBackend for B {
    fn propose(&mut self, _: VerifyWidth) -> Result<DraftProposal, SpeculativeError> {
        DraftProposal::new(vec![1, 2])
    }
    fn verify(&mut self, _: &DraftProposal) -> Result<Vec<u32>, SpeculativeError> {
        Ok(vec![1, 9])
    }
}
#[test]
fn recovers_with_target() {
    let r = run_cycle(&mut B, VerifyWidth::new(2).unwrap()).unwrap();
    assert_eq!(
        (r.verification.accepted, r.verification.fallback_token),
        (1, Some(9))
    );
}
