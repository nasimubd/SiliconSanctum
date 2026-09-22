use sanctum_runtime::tui::{CpuTelemetry,Utilization};
#[test]fn preserves_ecores(){let c=CpuTelemetry{performance:Utilization::new(0.8).unwrap(),efficiency:Utilization::new(0.2).unwrap()};assert!((c.efficiency.get()-0.2).abs()<f32::EPSILON);}
