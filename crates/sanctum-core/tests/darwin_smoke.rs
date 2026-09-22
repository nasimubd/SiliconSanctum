#![cfg(target_os = "macos")]

use sanctum_core::darwin::direct_io::DirectModelFile;
use sanctum_core::darwin::mach::{MachHost, NativeMachHost, memory_telemetry};
use sanctum_core::darwin::pressure::{MemoryPressure, MemoryPressureMonitor, PressureHandler};
use sanctum_core::darwin::qos::{NativeQosSetter, bind_inference_thread};
use sanctum_core::darwin::sysctl::{NativeSysctl, wired_limit_mb};

struct IgnorePressure;

impl PressureHandler for IgnorePressure {
    fn on_pressure(&self, _pressure: MemoryPressure) {}
}

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

#[test]
fn enables_no_cache_on_live_descriptor() {
    let fixture = tempfile::NamedTempFile::new().unwrap();
    let model = DirectModelFile::open(fixture.path()).unwrap();
    model.enable_no_cache().unwrap();
}

#[test]
fn creates_live_memory_pressure_source() {
    let monitor = MemoryPressureMonitor::start(IgnorePressure).unwrap();
    monitor.cancel();
}

#[test]
fn reads_live_iogpu_wired_limit() {
    assert!(wired_limit_mb(&NativeSysctl).is_ok());
}
