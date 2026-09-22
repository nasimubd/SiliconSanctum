use sanctum_runtime::prefill::{PrefillStep, SchedulerMetrics};

#[test]
fn records_scheduler_metrics() {
    let mut metrics = SchedulerMetrics::default();
    metrics.record_request();
    metrics.record_chunks(3);
    metrics.record_rejection();
    metrics.record_step(PrefillStep::Tokens512);
    metrics.record_step(PrefillStep::Tokens1024);
    assert_eq!(metrics.requests, 1);
    assert_eq!(metrics.chunks, 3);
    assert_eq!(metrics.rejected, 1);
    assert_eq!(metrics.selected_512, 1);
    assert_eq!(metrics.selected_1024, 1);
}
