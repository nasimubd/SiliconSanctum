use sanctum_runtime::migration::confirm_quiescence;

#[test]
fn requires_exact_confirmation() {
    assert!(confirm_quiescence("QUIESCED\n").is_ok());
    assert!(confirm_quiescence("quiesced\n").is_err());
    assert!(confirm_quiescence("QUIESCED ").is_err());
    assert!(confirm_quiescence("").is_err());
}
