#![no_main]

use libfuzzer_sys::fuzz_target;
use sanctum_core::darwin::mach::{MachHost, MachTelemetryError, VmPageCounters, memory_telemetry};

struct InputHost {
    page_size: u64,
    counters: VmPageCounters,
}

impl MachHost for InputHost {
    fn page_size(&self) -> Result<u64, MachTelemetryError> {
        Ok(self.page_size)
    }

    fn vm_counters(&self) -> Result<VmPageCounters, MachTelemetryError> {
        Ok(self.counters)
    }
}

fuzz_target!(|input: (u64, u64, u64, u64, u64, u64)| {
    let (page_size, free, active, inactive, wired, compressed) = input;
    let host = InputHost {
        page_size,
        counters: VmPageCounters {
            free,
            active,
            inactive,
            wired,
            compressed,
        },
    };
    if let Ok(snapshot) = memory_telemetry(&host) {
        assert_eq!(snapshot.free_bytes, free * page_size);
        assert_eq!(snapshot.active_bytes, active * page_size);
        assert_eq!(snapshot.inactive_bytes, inactive * page_size);
        assert_eq!(snapshot.wired_bytes, wired * page_size);
        assert_eq!(snapshot.compressed_bytes, compressed * page_size);
    }
});
