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
