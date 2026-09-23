use ratatui::{Terminal, backend::TestBackend};
use sanctum_runtime::tui::{render::render_dashboard, *};
#[test]
fn renders_memory() {
    let snapshot = DashboardSnapshot::new(
        TokenRate::new(0.0).unwrap(),
        MemoryTelemetry::new(MemoryBytes(4), MemoryBytes(2), MemoryBytes(16)).unwrap(),
        CpuTelemetry {
            performance: Utilization::new(0.5).unwrap(),
            efficiency: Utilization::new(0.2).unwrap(),
        },
        KvResidency::new(1, 4).unwrap(),
        ActiveProfile::new("focused").unwrap(),
    );
    let mut state = DashboardState::new(snapshot);
    state.tab = DashboardTab::Memory;
    let mut terminal = Terminal::new(TestBackend::new(80, 28)).unwrap();
    terminal
        .draw(|frame| render_dashboard(frame, &state, &MetricHistory::new(8).unwrap()))
        .unwrap();
    assert!(format!("{:?}", terminal.backend().buffer()).contains("Wired memory"));
}
