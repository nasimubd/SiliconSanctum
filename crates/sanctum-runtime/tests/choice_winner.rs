use sanctum_runtime::decision::*;
#[test]
fn resolves_choice_winner() {
    let ids = vec![ChoiceId::new("a").unwrap(), ChoiceId::new("b").unwrap()];
    let value = Choice::from_logits(ids, vec![0.0, 2.0]).unwrap();
    assert_eq!(value.winner().id.as_str(), "b");
}
