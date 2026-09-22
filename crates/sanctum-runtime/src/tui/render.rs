//! Ratatui views for runtime telemetry.
use ratatui::layout::Rect;
use ratatui::Frame;
use super::{DashboardState,MetricHistory};
pub fn render_dashboard(_frame:&mut Frame<'_>,_state:&DashboardState,_history:&MetricHistory){}
pub fn body_area(area:Rect)->Rect{area}
