//! Ratatui views for runtime telemetry.
use super::{DashboardState, DashboardTab, MetricHistory};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Gauge, Paragraph, Sparkline};
pub fn render_dashboard(frame: &mut Frame<'_>, state: &DashboardState, history: &MetricHistory) {
    let layout = regions(frame.area());
    render_header(frame, layout.header, state);
    match state.tab {
        DashboardTab::Overview => render_overview(frame, layout.body, state, history),
        DashboardTab::Memory => render_memory(frame, layout.body, state),
        DashboardTab::Compute => render_compute(frame, layout.body, state),
    }
    render_footer(frame, layout.footer);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DashboardRegions {
    pub header: Rect,
    pub body: Rect,
    pub footer: Rect,
}
pub fn regions(area: Rect) -> DashboardRegions {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(area);
    DashboardRegions {
        header: chunks[0],
        body: chunks[1],
        footer: chunks[2],
    }
}
pub fn body_area(area: Rect) -> Rect {
    regions(area).body
}
pub fn render_header(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let status = if state.paused { "PAUSED" } else { "LIVE" };
    let title = format!(
        "Silicon Sanctum  |  {}  |  {}",
        state.snapshot.profile.as_str(),
        status
    );
    frame.render_widget(
        Paragraph::new(title).block(Block::default().borders(Borders::ALL)),
        area,
    );
}
pub fn render_footer(frame: &mut Frame<'_>, area: Rect) {
    frame.render_widget(
        Paragraph::new("q quit  |  space pause  |  tab view  |  r refresh"),
        area,
    );
}
pub fn render_token_rate(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    frame.render_widget(
        Paragraph::new(format!(
            "Generation: {:.1} tok/s",
            state.snapshot.token_rate.get()
        )),
        area,
    );
}
pub fn render_memory(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);
    let memory = state.snapshot.memory;
    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("Wired memory").borders(Borders::ALL))
            .ratio(memory.wired.ratio(memory.total).clamp(0.0, 1.0)),
        rows[0],
    );
    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("OS cache").borders(Borders::ALL))
            .ratio(memory.os_cache.ratio(memory.total).clamp(0.0, 1.0)),
        rows[1],
    );
    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("KV slots").borders(Borders::ALL))
            .ratio(f64::from(state.snapshot.kv.ratio()).clamp(0.0, 1.0)),
        rows[2],
    );
}
pub fn render_compute(frame: &mut Frame<'_>, area: Rect, state: &DashboardState) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Length(3)])
        .split(area);
    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("P cores").borders(Borders::ALL))
            .ratio(f64::from(state.snapshot.cpu.performance.get())),
        rows[0],
    );
    frame.render_widget(
        Gauge::default()
            .block(Block::default().title("E cores").borders(Borders::ALL))
            .ratio(f64::from(state.snapshot.cpu.efficiency.get())),
        rows[1],
    );
}
pub fn render_history(frame: &mut Frame<'_>, area: Rect, history: &MetricHistory) {
    let samples: Vec<u64> = history
        .token_rates()
        .iter()
        .map(|rate| *rate as u64)
        .collect();
    frame.render_widget(
        Sparkline::default()
            .block(
                Block::default()
                    .title("Token rate history")
                    .borders(Borders::ALL),
            )
            .data(&samples),
        area,
    );
}
pub fn render_overview(
    frame: &mut Frame<'_>,
    area: Rect,
    state: &DashboardState,
    history: &MetricHistory,
) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Length(9),
            Constraint::Length(6),
            Constraint::Min(1),
        ])
        .split(area);
    render_token_rate(frame, rows[0], state);
    render_memory(frame, rows[1], state);
    render_compute(frame, rows[2], state);
    render_history(frame, rows[3], history);
}
