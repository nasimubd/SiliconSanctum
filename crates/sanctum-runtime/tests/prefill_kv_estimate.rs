use sanctum_runtime::prefill::{KvQuantization, ModelGeometry};

#[test]
fn estimates_quantized_kv_bytes() {
    let geometry = ModelGeometry::new(2, 2, 4).unwrap();
    assert_eq!(geometry.kv_bytes_per_token(KvQuantization::Bits4), 16);
    assert_eq!(geometry.kv_bytes_per_token(KvQuantization::Bits16), 64);
}
