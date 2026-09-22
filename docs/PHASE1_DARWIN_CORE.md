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
source exactly once. An unwinding handler panic is contained at the C ABI boundary,
recorded by `callback_panicked`, and disables subsequent callback delivery. A
`panic=abort` build still terminates by definition.

## Thread quality of service

Inference workers call `pthread_set_qos_class_self_np` with
`QOS_CLASS_USER_INTERACTIVE` and zero relative priority. This is Darwin's strongest
latency scheduling hint; it is not a hard CPU-affinity API and does not guarantee
that every scheduling quantum runs on a performance core.

## Direct model I/O

Model descriptors are opened read-only and configured with `F_NOCACHE` before
streaming. Buffers come from `posix_memalign` at a 16,384-byte boundary and remain
uniquely owned until release. `pread` preserves descriptor position, retries
interruptions, and reports short reads without fabricating bytes.

`NativeSharedBuffer` owns a page-aligned CPU allocation and a no-copy
`MTLResourceStorageModeShared` buffer over the same bytes. The Metal object is
released before its backing allocation. Resource access is unsafe because callers
must finish GPU commands before CPU access or owner destruction.

Transfers reject undersized or misaligned storage and require complete requested
ranges. Bounded scoped workers borrow only exclusive CPU slices, preserve request
order, limit live threads, and preflight the page-rounded retained allocation
against the caller's memory budget.

## Validation matrix

| Surface | Validation |
| --- | --- |
| Sysctl codec and guard | Unit, property, and live read smoke tests |
| Mach page conversion | Unit, overflow, and live host telemetry tests |
| Dispatch pressure | Flag/callback unit tests and live source lifecycle test |
| Thread QoS | Parameter unit test and live pthread binding test |
| Direct I/O | Alignment, boundary, concurrency, property, and live F_NOCACHE tests |
| Native Metal storage | Shared pointer, order, truncation, and budget integration tests |
| Fuzzing | libFuzzer targets for layouts, telemetry arithmetic, and sysctl payloads |
| Toolchain | rustfmt, Cargo check, Clippy with warnings denied, and Cargo test |

The live tests are compiled only for macOS. CI runs the complete suite on an
Apple Silicon `macos-14` runner so the Darwin FFI paths are compiled and invoked.
CI also performs a bounded smoke run of every fuzz target under nightly Rust.
