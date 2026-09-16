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
