use sanctum_runtime::tui::{MemoryBytes,MemoryTelemetry,MetricSeverity};
#[test]fn warns_at_ceiling(){let memory=MemoryTelemetry::new(MemoryBytes(95),MemoryBytes(0),MemoryBytes(100)).unwrap();assert_eq!(memory.severity(),MetricSeverity::Critical);}
