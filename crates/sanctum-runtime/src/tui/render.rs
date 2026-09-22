//! Ratatui views for runtime telemetry.
use ratatui::layout::{Constraint,Direction,Layout,Rect};
use ratatui::Frame;
use ratatui::widgets::{Block,Borders,Paragraph};
use super::{DashboardState,MetricHistory};
pub fn render_dashboard(_frame:&mut Frame<'_>,_state:&DashboardState,_history:&MetricHistory){}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct DashboardRegions{pub header:Rect,pub body:Rect,pub footer:Rect}
pub fn regions(area:Rect)->DashboardRegions{let chunks=Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3),Constraint::Min(1),Constraint::Length(2)]).split(area);DashboardRegions{header:chunks[0],body:chunks[1],footer:chunks[2]}}
pub fn body_area(area:Rect)->Rect{regions(area).body}
pub fn render_header(frame:&mut Frame<'_>,area:Rect,state:&DashboardState){let status=if state.paused{"PAUSED"}else{"LIVE"};let title=format!("Silicon Sanctum  |  {}  |  {}",state.snapshot.profile.as_str(),status);frame.render_widget(Paragraph::new(title).block(Block::default().borders(Borders::ALL)),area);}
