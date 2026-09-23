use sanctum_runtime::migration::{BusInspection,LinkSpeed};
#[test]fn accepts_x4_8gt(){let report="Thunderbolt/USB4:\n  Port:\n    Status: Device connected\n    Link Width: x4\n    Link Speed: 8.0 GT/s";let link=BusInspection::parse(report).unwrap();assert!(link.qualifies());assert_eq!(link.speed,LinkSpeed::Gt8);}
