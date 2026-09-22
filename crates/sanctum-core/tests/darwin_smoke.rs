#![cfg(target_os = "macos")]

use sanctum_core::darwin::mach::{MachHost, NativeMachHost, memory_telemetry};
use sanctum_core::darwin::qos::{NativeQosSetter, bind_inference_thread};

#[test]
fn reads_live_mach_memory_telemetry() {
    let telemetry = memory_telemetry(&NativeMachHost).unwrap();
    assert!(telemetry.page_size > 0);
    assert!(telemetry.wired_bytes > 0);
    assert!(NativeMachHost.vm_counters().is_ok());
}

#[test]
fn binds_live_thread_qos() {
    bind_inference_thread(&NativeQosSetter).unwrap();
}
