use sanctum_runtime::tui::{MemoryBytes,MemoryTelemetry};
#[test]fn rejects_combined_overcommit(){assert!(MemoryTelemetry::new(MemoryBytes(10),MemoryBytes(7),MemoryBytes(16)).is_err());}
