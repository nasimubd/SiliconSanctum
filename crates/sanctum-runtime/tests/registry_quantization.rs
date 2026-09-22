use sanctum_runtime::registry::Quantization;
#[test]
fn reports_quantization_bits() {
    assert_eq!(Quantization::Q4.bits(), 4);
    assert_eq!(Quantization::F16.bits(), 16);
}
