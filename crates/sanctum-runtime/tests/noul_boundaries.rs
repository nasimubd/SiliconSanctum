use sanctum_runtime::decision::Noul;
#[test]
fn validates_noul_boundaries() {
    assert!(Noul::new(0.0).is_ok());
    assert!(Noul::new(1.0).is_ok());
    assert!(Noul::new(-0.1).is_err());
    assert!(Noul::new(1.1).is_err());
}
