use sanctum_runtime::prefill::KvQuantization;

#[test]
fn reports_quantization_widths() {
    assert_eq!(KvQuantization::Bits4.bits(), 4);
    assert_eq!(KvQuantization::Bits8.bits(), 8);
    assert_eq!(KvQuantization::Bits16.bits(), 16);
}
