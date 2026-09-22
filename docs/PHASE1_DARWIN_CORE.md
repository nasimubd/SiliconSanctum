# Phase 1 Darwin Core

## IOGPU wired limit

The sysctl layer reads and writes `iogpu.wired_limit_mb` using native-width byte
payloads. Applying a temporary ceiling captures the previous value and returns a
guard that restores it on drop. Callers that need restoration errors must invoke
the explicit `restore` method before releasing the guard.

Changing this sysctl generally requires elevated privileges. A failed update is
reported without changing the captured prior value. Operators must size the
ceiling below the host's physical-memory limit and retain headroom for macOS.

## Mach telemetry

The telemetry snapshot combines `host_statistics64` counters with Mach's native
VM page size. Free, active, inactive, wired, and compressor-resident page counts
are converted with checked multiplication. Counter overflow fails the complete
snapshot rather than publishing wrapped memory figures.

The wired value describes host wired pages, while the compressed value describes
pages resident in the compressor. Neither value is interchangeable with process
resident size or the configured IOGPU ceiling.

## Memory pressure

The monitor owns a `DISPATCH_SOURCE_TYPE_MEMORYPRESSURE` source covering normal,
warning, and critical events. Warning callbacks are the early-eviction signal;
critical callbacks require immediate eviction. The event context remains owned by
libdispatch until source finalization, and monitor drop cancels and releases the
source exactly once.

## Thread quality of service

Inference workers call `pthread_set_qos_class_self_np` with
`QOS_CLASS_USER_INTERACTIVE` and zero relative priority. This is Darwin's strongest
latency scheduling hint; it is not a hard CPU-affinity API and does not guarantee
that every scheduling quantum runs on a performance core.
