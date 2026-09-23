use sanctum_runtime::migration::Sha256Digest;
#[test]
fn formats_64_hex_digits() {
    assert_eq!(Sha256Digest([0; 32]).to_hex(), "0".repeat(64));
}
