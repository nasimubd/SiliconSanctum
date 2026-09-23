use sanctum_runtime::migration::supports_required_rsync_flags;
#[test]
fn rejects_missing_x() {
    assert!(!supports_required_rsync_flags(
        "usage: rsync [-0468BCDEFHIKLOPRSTWVabcd]"
    ));
    assert!(supports_required_rsync_flags("usage: rsync [-avXHE]"));
}
