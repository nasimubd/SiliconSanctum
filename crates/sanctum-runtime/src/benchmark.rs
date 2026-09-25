//! Small, repeatable host probe used by installation and doctor commands.
#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use std::{fs, time::Instant};

#[derive(Debug, Clone, Serialize)]
pub struct HardwareReport {
    pub platform: String,
    pub logical_cpus: usize,
    pub memory_bytes: Option<u64>,
    pub storage_probe_ms: u128,
    pub recommendation: &'static str,
}

#[must_use]
pub fn probe() -> HardwareReport {
    let logical_cpus = std::thread::available_parallelism().map_or(1, usize::from);
    let memory_bytes = memory_bytes();
    let started = Instant::now();
    let _ = fs::read_dir(".").map(std::iter::Iterator::count);
    let storage_probe_ms = started.elapsed().as_millis();
    let recommendation = match memory_bytes {
        Some(bytes) if bytes >= 32 * 1024 * 1024 * 1024 => "quality-and-long-context",
        Some(bytes) if bytes >= 16 * 1024 * 1024 * 1024 => "small-and-mid-size-models",
        Some(_) => "small-models-only",
        None => "run-backend-benchmark",
    };
    HardwareReport {
        platform: std::env::consts::OS.to_owned(),
        logical_cpus,
        memory_bytes,
        storage_probe_ms,
        recommendation,
    }
}

fn memory_bytes() -> Option<u64> {
    #[cfg(target_os = "macos")]
    {
        let output = std::process::Command::new("sysctl")
            .args(["-n", "hw.memsize"])
            .output()
            .ok()?;
        return String::from_utf8(output.stdout).ok()?.trim().parse().ok();
    }
    #[cfg(target_os = "linux")]
    {
        let text = fs::read_to_string("/proc/meminfo").ok()?;
        let kib = text
            .lines()
            .find(|line| line.starts_with("MemTotal:"))?
            .split_whitespace()
            .nth(1)?
            .parse::<u64>()
            .ok()?;
        return Some(kib.saturating_mul(1024));
    }
    #[allow(unreachable_code)]
    None
}
