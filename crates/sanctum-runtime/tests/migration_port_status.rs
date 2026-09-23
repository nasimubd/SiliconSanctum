use sanctum_runtime::migration::PortStatus;
#[test]fn rejects_no_device(){assert_eq!(PortStatus::parse("No device connected"),PortStatus::Disconnected);}
