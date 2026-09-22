use sanctum_runtime::tui::{KvResidency,MetricSeverity};
#[test]fn flags_full_cache(){assert_eq!(KvResidency::new(10,10).unwrap().severity(),MetricSeverity::Critical);}
