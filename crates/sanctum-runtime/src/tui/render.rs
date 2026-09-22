//! Ratatui views for runtime telemetry.
use ratatui::layout::{Constraint,Direction,Layout,Rect};
use ratatui::Frame;
use ratatui::widgets::{Block,Borders,Gauge,Paragraph};
use super::{DashboardState,MetricHistory};
pub fn render_dashboard(_frame:&mut Frame<'_>,_state:&DashboardState,_history:&MetricHistory){}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct DashboardRegions{pub header:Rect,pub body:Rect,pub footer:Rect}
pub fn regions(area:Rect)->DashboardRegions{let chunks=Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3),Constraint::Min(1),Constraint::Length(2)]).split(area);DashboardRegions{header:chunks[0],body:chunks[1],footer:chunks[2]}}
pub fn body_area(area:Rect)->Rect{regions(area).body}
pub fn render_header(frame:&mut Frame<'_>,area:Rect,state:&DashboardState){let status=if state.paused{"PAUSED"}else{"LIVE"};let title=format!("Silicon Sanctum  |  {}  |  {}",state.snapshot.profile.as_str(),status);frame.render_widget(Paragraph::new(title).block(Block::default().borders(Borders::ALL)),area);}
pub fn render_footer(frame:&mut Frame<'_>,area:Rect){frame.render_widget(Paragraph::new("q quit  |  space pause  |  tab view  |  r refresh"),area);}
pub fn render_token_rate(frame:&mut Frame<'_>,area:Rect,state:&DashboardState){frame.render_widget(Paragraph::new(format!("Generation: {:.1} tok/s",state.snapshot.token_rate.get())),area);}
pub fn render_memory(frame:&mut Frame<'_>,area:Rect,state:&DashboardState){let rows=Layout::default().direction(Direction::Vertical).constraints([Constraint::Length(3),Constraint::Length(3),Constraint::Length(3)]).split(area);let memory=state.snapshot.memory;frame.render_widget(Gauge::default().block(Block::default().title("Wired memory").borders(Borders::ALL)).ratio(memory.wired.ratio(memory.total).clamp(0.0,1.0)),rows[0]);frame.render_widget(Gauge::default().block(Block::default().title("OS cache").borders(Borders::ALL)).ratio(memory.os_cache.ratio(memory.total).clamp(0.0,1.0)),rows[1]);frame.render_widget(Gauge::default().block(Block::default().title("KV slots").borders(Borders::ALL)).ratio(f64::from(state.snapshot.kv.ratio()).clamp(0.0,1.0)),rows[2]);}
