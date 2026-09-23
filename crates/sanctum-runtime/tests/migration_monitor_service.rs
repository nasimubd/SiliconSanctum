use sanctum_runtime::migration::{MONITOR_SERVICE_LABEL, MonitorService};
use std::path::Path;

#[test]
fn renders_restartable_monitor_arguments() {
    let service = MonitorService {
        executable: Path::new("/Applications/Sanctum & Tools/sanctum-migrate"),
        record: Path::new("/tmp/record.plist"),
        pointer: Path::new("/tmp/root"),
    };
    let document = service.render_plist().unwrap();
    let value = plist::Value::from_reader_xml(document.as_bytes()).unwrap();
    let dictionary = value.as_dictionary().unwrap();
    assert_eq!(dictionary["Label"].as_string(), Some(MONITOR_SERVICE_LABEL));
    assert_eq!(dictionary["RunAtLoad"].as_boolean(), Some(true));
    assert_eq!(
        dictionary["KeepAlive"].as_dictionary().unwrap()["SuccessfulExit"].as_boolean(),
        Some(false)
    );
    let arguments = dictionary["ProgramArguments"].as_array().unwrap();
    assert_eq!(
        arguments[0].as_string(),
        Some("/Applications/Sanctum & Tools/sanctum-migrate")
    );
    assert_eq!(arguments[1].as_string(), Some("monitor"));
    assert_eq!(arguments[2].as_string(), Some("/tmp/record.plist"));
    assert_eq!(arguments[3].as_string(), Some("/tmp/root"));
}
