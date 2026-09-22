//! Grand Central Dispatch memory-pressure monitoring.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}

pub const PRESSURE_NORMAL: usize = 0x01;
pub const PRESSURE_WARNING: usize = 0x02;
pub const PRESSURE_CRITICAL: usize = 0x04;

#[must_use]
pub const fn decode_pressure(flags: usize) -> MemoryPressure {
    if flags & PRESSURE_CRITICAL != 0 {
        MemoryPressure::Critical
    } else if flags & PRESSURE_WARNING != 0 {
        MemoryPressure::Warning
    } else {
        MemoryPressure::Normal
    }
}

pub trait PressureHandler: Send + Sync + 'static {
    fn on_pressure(&self, pressure: MemoryPressure);
}

pub fn dispatch_pressure(handler: &impl PressureHandler, flags: usize) {
    handler.on_pressure(decode_pressure(flags));
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::{
        MemoryPressure, PRESSURE_CRITICAL, PRESSURE_NORMAL, PRESSURE_WARNING, PressureHandler,
        decode_pressure, dispatch_pressure,
    };

    #[derive(Default)]
    struct Recorder(Mutex<Vec<MemoryPressure>>);

    impl PressureHandler for Recorder {
        fn on_pressure(&self, pressure: MemoryPressure) {
            self.0.lock().unwrap().push(pressure);
        }
    }

    #[test]
    fn invokes_warning_callback() {
        let recorder = Recorder::default();
        dispatch_pressure(&recorder, PRESSURE_WARNING);
        assert_eq!(*recorder.0.lock().unwrap(), [MemoryPressure::Warning]);
    }

    #[test]
    fn invokes_critical_callback() {
        let recorder = Recorder::default();
        dispatch_pressure(&recorder, PRESSURE_CRITICAL);
        assert_eq!(*recorder.0.lock().unwrap(), [MemoryPressure::Critical]);
    }

    #[test]
    fn decodes_normal_event() {
        assert_eq!(decode_pressure(PRESSURE_NORMAL), MemoryPressure::Normal);
    }

    #[test]
    fn decodes_warning_event() {
        assert_eq!(decode_pressure(PRESSURE_WARNING), MemoryPressure::Warning);
    }

    #[test]
    fn decodes_critical_event() {
        assert_eq!(decode_pressure(PRESSURE_CRITICAL), MemoryPressure::Critical);
    }
}
