# Phase 5 Part 3 completion

- [x] Inspect Thunderbolt/USB4 ports for a connected PCIe x4 link at 8.0 or 16.0 GT/s.
- [x] Require recent positive APFS spaceman TRIM evidence.
- [x] Refuse rsync binaries that do not advertise the required `-avXHE` flags.
- [x] Run initial and final attribute-preserving transfer passes.
- [x] Compare deterministic SHA-256 manifests before cutover.
- [x] Persist cutover state and switch the `AI_ROOT` pointer atomically.
- [x] Monitor link, target availability, and kernel panic evidence for 72 hours.
- [x] Restore the origin pointer automatically on detected failure.
- [x] Provide inspect, dry-run, execute, and monitor commands.
- [x] Pass workspace formatting, check, test, and lint gates.

The reference host currently has no connected Thunderbolt storage device and
its bundled rsync lacks `-X`; a physical migration cannot be exercised there.
