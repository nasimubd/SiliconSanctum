use sanctum_runtime::gateway::counting_marker_count;
#[test]
fn counts_counting_markers() {
    assert_eq!(counting_marker_count("count how many"), 2);
}
