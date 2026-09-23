use sanctum_runtime::migration::hash_file;
#[test]
fn hashes_abc() {
    let file = tempfile::NamedTempFile::new().unwrap();
    std::fs::write(file.path(), b"abc").unwrap();
    assert_eq!(
        hash_file(file.path()).unwrap().to_hex(),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}
