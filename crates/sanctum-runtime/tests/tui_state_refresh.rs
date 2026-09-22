use sanctum_runtime::tui::*;
struct Source(DashboardSnapshot);
impl TelemetrySource for Source{fn snapshot(&mut self)->Result<DashboardSnapshot,DashboardError>{Ok(self.0.clone())}}
#[test]fn refreshes_snapshot(){let mk=|rate|DashboardSnapshot::new(TokenRate::new(rate).unwrap(),MemoryTelemetry::new(MemoryBytes(0),MemoryBytes(0),MemoryBytes(1)).unwrap(),CpuTelemetry{performance:Utilization::new(0.0).unwrap(),efficiency:Utilization::new(0.0).unwrap()},KvResidency::new(0,1).unwrap(),ActiveProfile::new("idle").unwrap());let mut state=DashboardState::new(mk(1.0));state.refresh(&mut Source(mk(2.0))).unwrap();assert!((state.snapshot.token_rate.get()-2.0).abs()<f64::EPSILON);}
