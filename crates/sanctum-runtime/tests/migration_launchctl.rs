use sanctum_runtime::migration::launchctl_bootstrap_arguments;
use std::path::Path;

#[test]
fn bootstraps_the_current_user_domain_without_shell_expansion() {
    let arguments = launchctl_bootstrap_arguments(501, Path::new("/tmp/LaunchAgents/a & b.plist"));
    assert_eq!(arguments[0], "bootstrap");
    assert_eq!(arguments[1], "gui/501");
    assert_eq!(arguments[2], "/tmp/LaunchAgents/a & b.plist");
}
