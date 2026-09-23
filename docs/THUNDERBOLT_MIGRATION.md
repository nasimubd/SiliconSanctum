# Thunderbolt cable migration

The current Transcend `TS-CM10G` device is enumerating on the USB 3.1 bus and repeated cold reads were approximately 40 MB/s. A 40 Gb/s cable alone does not guarantee a 40 Gb/s storage link: the enclosure, controller, cable, filesystem, thermal state, and negotiated protocol must all support it. A 10 Gb/s enclosure remains limited to 10 Gb/s even on a 40 Gb/s cable.

When the new cable arrives:

1. Stop inference and backtests; unmount the volume normally.
2. Connect the enclosure directly without a hub.
3. Confirm whether it enumerates under Thunderbolt/USB4 or USB and record negotiated speed.
4. Run `./scripts/storage-benchmark.sh 16` twice, retaining both JSON results.
5. Compare sustained read/write rates and enclosure temperature.
6. Do not change model profiles until the new path is stable across sleep/wake and a multi-hour workload.

## Recorded result — 2026-09-16

The new cable did **not** migrate TickArchive to Thunderbolt/USB4. macOS reports
the Transcend `TS-CM10G` on USB 3.1 at up to 10 Gb/s; all three
Thunderbolt/USB4 ports report no connected device. The enclosure/controller is
therefore the bottleneck, as expected for this class of USB enclosure.

Two direct 16 GiB passes on `/Volumes/TickArchive` completed successfully:

| Pass | Write | Read |
|---|---:|---:|
| 1 | 983.4 MB/s | 977.1 MB/s |
| 2 | 1001.4 MB/s | 979.1 MB/s |

These are storage-throughput measurements, not cold model-load measurements.
Run `local-ai migrate-thunderbolt` after future hardware changes; it records
the detected transport in each result and retains two benchmark JSON records.
It does not unmount the drive or stop workloads automatically.

## Verified Rust migration

The `sanctum-migrate` binary provides the transfer and rollback workflow. Build
it with `cargo build --release -p sanctum-runtime --bin sanctum-migrate`, then
run `inspect` and `dry-run ORIGIN TARGET RSYNC` before `execute ORIGIN TARGET
RSYNC RECORD POINTER SERVICE_PLIST`. Use absolute paths. `TARGET` must be an empty directory
on a different mounted volume. Set `POINTER` to this repository's
`.state/ai-root` and keep `RECORD` in persistent storage. Set `SERVICE_PLIST`
to the absolute path `~/Library/LaunchAgents/com.siliconsanctum.migration-monitor.plist`,
expanding `~` before invoking the command. The service path must not already exist.

`execute` requires a connected Thunderbolt/USB4 port with a reported PCIe x4
link at 8.0 or 16.0 GT/s, recent positive APFS spaceman TRIM evidence naming
the destination volume UUID or device identifier, and an
rsync binary supporting `-avXHE`. The system `/usr/bin/rsync` on the reference
host does not advertise `-X`; use a compatible rsync installation. The command
runs two passes, compares SHA-256 manifests, then switches `AI_ROOT`. It does
not remove the origin. After the initial pass, stop all writers to the origin
and type the exact confirmation `QUIESCED`. The final pass checks that the
origin manifest remains unchanged while it runs; resume writers only after
cutover succeeds. A changed source aborts the cutover.

After cutover, the native monitor checks the link, target mount, and recent
kernel panic logs every ten seconds for 72 hours. If one fails, it restores
the origin pointer. Keep the origin volume mounted and retain the monitor
binary for the entire window. The native monitor is registered with the current
user's launchd domain: abnormal exits restart it, and it starts again at the
next user login after a reboot. It does not run before login, and a panic older
than the monitor's five-minute log query can be missed. Inspect the cutover
record and pointer after a reboot before starting workloads or removing the
origin volume. The legacy launcher reads the pointer unless
`AI_ROOT` was explicitly set in the process environment.
