use sanctum_runtime::migration::{MonitorService, persist_monitor_service};
use std::path::Path;

#[test]
fn writes_only_a_new_launch_agent_file() {
    let temporary = tempfile::tempdir().unwrap();
    let service = MonitorService {
        executable: Path::new("/usr/local/bin/sanctum-migrate"),
        record: Path::new("/tmp/record"),
        pointer: Path::new("/tmp/pointer"),
    };
    let misplaced = temporary.path().join("monitor.plist");
    assert!(persist_monitor_service(&service, &misplaced).is_err());
    let destination = temporary.path().join("LaunchAgents/com.siliconsanctum.migration-monitor.plist");
    persist_monitor_service(&service, &destination).unwrap();
    assert!(destination.is_file());
    assert!(persist_monitor_service(&service, &destination).is_err());
}
