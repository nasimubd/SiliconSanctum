# Phase 1 Darwin Core

## IOGPU wired limit

The sysctl layer reads and writes `iogpu.wired_limit_mb` using native-width byte
payloads. Applying a temporary ceiling captures the previous value and returns a
guard that restores it on drop. Callers that need restoration errors must invoke
the explicit `restore` method before releasing the guard.

Changing this sysctl generally requires elevated privileges. A failed update is
reported without changing the captured prior value. Operators must size the
ceiling below the host's physical-memory limit and retain headroom for macOS.
