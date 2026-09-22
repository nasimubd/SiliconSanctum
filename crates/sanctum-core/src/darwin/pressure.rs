//! Grand Central Dispatch memory-pressure monitoring.

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MemoryPressure {
    Normal,
    Warning,
    Critical,
}
