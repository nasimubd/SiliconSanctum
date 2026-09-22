use sanctum_runtime::decision::*;
#[test]
fn normalizes_choice_probabilities() {
    let ids = vec![ChoiceId::new("a").unwrap(), ChoiceId::new("b").unwrap()];
    let value = Choice::from_logits(ids, vec![0.0, 0.0]).unwrap();
    let sum: f64 = value.items().iter().map(|item| item.probability).sum();
    assert!((sum - 1.0).abs() < 1.0e-12);
}
