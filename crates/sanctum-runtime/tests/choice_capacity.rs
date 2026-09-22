use sanctum_runtime::decision::*;
#[test]
fn enforces_choice_capacity() {
    let ids = (0..256)
        .map(|i| ChoiceId::new(i.to_string()).unwrap())
        .collect();
    assert!(Choice::from_logits(ids, vec![0.0; 256]).is_err());
}
