use sanctum_runtime::speculative::AcceptanceMetrics;
#[test] fn empty_rate_is_zero(){assert_eq!(AcceptanceMetrics::default().rate(),0.0);}
