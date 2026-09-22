use sanctum_runtime::gateway::*;
#[test]
fn deduplicates_evidence() {
    let mut r = JaggednessReport::clear();
    let e = Evidence::new(JaggednessKind::Counting, "count").unwrap();
    r.push(e.clone());
    r.push(e);
    assert_eq!(r.evidence().len(), 1);
}
