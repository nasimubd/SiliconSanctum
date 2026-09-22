#![cfg(target_os = "macos")]

use sanctum_core::darwin::mach::{MachHost, NativeMachHost, memory_telemetry};

#[test]
fn reads_live_mach_memory_telemetry() {
    let telemetry = memory_telemetry(&NativeMachHost).unwrap();
    assert!(telemetry.page_size > 0);
    assert!(telemetry.wired_bytes > 0);
    assert!(NativeMachHost.vm_counters().is_ok());
}
