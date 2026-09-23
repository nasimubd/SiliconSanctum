use sanctum_runtime::migration::LinkWidth;
#[test]
fn parses_x4() {
    assert_eq!(LinkWidth::parse("x4").unwrap().lanes(), 4);
}
