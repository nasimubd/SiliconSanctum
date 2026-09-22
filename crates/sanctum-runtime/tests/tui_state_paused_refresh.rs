use sanctum_runtime::tui::*;
struct FailingSource;
impl TelemetrySource for FailingSource{fn snapshot(&mut self)->Result<DashboardSnapshot,DashboardError>{Err(DashboardError::InvalidMetric)}}
#[test]fn skips_source_when_paused(){let initial=DashboardSnapshot::new(TokenRate::new(1.0).unwrap(),MemoryTelemetry::new(MemoryBytes(0),MemoryBytes(0),MemoryBytes(1)).unwrap(),CpuTelemetry{performance:Utilization::new(0.0).unwrap(),efficiency:Utilization::new(0.0).unwrap()},KvResidency::new(0,1).unwrap(),ActiveProfile::new("idle").unwrap());let mut state=DashboardState::new(initial.clone());state.paused=true;state.refresh(&mut FailingSource).unwrap();assert_eq!(state.snapshot,initial);}
