#![no_main]

use libfuzzer_sys::fuzz_target;
use sanctum_core::darwin::sysctl::{SysctlError, SysctlRead, wired_limit_mb};

struct Payload(Vec<u8>);

impl SysctlRead for Payload {
    fn read(&self, _key: &str) -> Result<Vec<u8>, SysctlError> {
        Ok(self.0.clone())
    }
}

fuzz_target!(|bytes: &[u8]| {
    let result = wired_limit_mb(&Payload(bytes.to_vec()));
    match bytes.len() {
        4 => assert_eq!(
            result.unwrap(),
            u64::from(u32::from_ne_bytes(bytes.try_into().unwrap()))
        ),
        8 => assert_eq!(
            result.unwrap(),
            u64::from_ne_bytes(bytes.try_into().unwrap())
        ),
        _ => assert!(
            matches!(result, Err(SysctlError::InvalidWidth { actual, .. }) if actual == bytes.len())
        ),
    }
});
