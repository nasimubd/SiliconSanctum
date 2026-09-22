use sanctum_runtime::speculative::ModelProfile;
#[test]fn labels_pair(){assert_eq!(ModelProfile{draft:"1.5B".into(),target:"8B".into()}.label(),"1.5B -> 8B");}
