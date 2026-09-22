use sanctum_runtime::speculative::TimingSample;
#[test]fn calculates_rate(){assert_eq!(TimingSample{tokens:20,elapsed_seconds:2.0}.tokens_per_second(),10.0);}
