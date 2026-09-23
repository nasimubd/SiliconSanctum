use sanctum_runtime::migration::TargetVolumeIdentity;
#[test]fn rejects_non_apfs(){let xml=br#"<?xml version="1.0"?><plist version="1.0"><dict><key>FilesystemType</key><string>exfat</string><key>VolumeUUID</key><string>98927CDF-4342-412B-AB62-231E18833833</string><key>DeviceIdentifier</key><string>disk5s1</string></dict></plist>"#;assert!(TargetVolumeIdentity::parse_plist(xml).is_err());}
