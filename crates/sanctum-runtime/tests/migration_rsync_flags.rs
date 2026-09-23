use sanctum_runtime::migration::{MigrationPaths, RsyncInvocation, SyncPass};
use std::path::PathBuf;
#[test]
fn uses_attribute_flags() {
    let paths = MigrationPaths::new(
        PathBuf::from("/Volumes/Origin/data"),
        PathBuf::from("/Volumes/Target/data"),
    )
    .unwrap();
    let call = RsyncInvocation::new(
        PathBuf::from("/opt/homebrew/bin/rsync"),
        &paths,
        SyncPass::Initial,
    );
    assert_eq!(call.arguments[0], "-avXHE");
    assert_eq!(call.arguments[1], "--");
}
