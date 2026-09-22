use sanctum_runtime::gateway::temporal_marker_count;
#[test]
fn counts_temporal_markers() {
    assert_eq!(temporal_marker_count("before and after"), 2);
}
