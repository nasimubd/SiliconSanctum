use sanctum_runtime::prefill::{PrefillError, ScratchBudget};

#[test]
fn rejects_zero_scratch_budget() {
    assert_eq!(ScratchBudget::new(0), Err(PrefillError::ZeroBudget));
}
