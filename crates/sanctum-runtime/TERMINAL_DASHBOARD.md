# Terminal dashboard

The dashboard shows token generation rate, wired memory, OS cache, P-core and E-core utilization, KV slot residency, and the active model profile. Press Tab to cycle overview, memory, and compute views; Space pauses automatic refresh; R refreshes immediately; Q or Escape exits. The terminal session restores normal mode on exit or error.

Call `run_dashboard` with a source implementing `TelemetrySource` and a nonzero `RefreshRate`. `SharedTelemetrySource` lets a sampler publish validated snapshots from another thread. The dashboard reads the latest snapshot at each refresh interval; producers remain responsible for measuring the host and inference process.
