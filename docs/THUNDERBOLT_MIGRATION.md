# Thunderbolt cable migration

The current Transcend `TS-CM10G` device is enumerating on the USB 3.1 bus and repeated cold reads were approximately 40 MB/s. A 40 Gb/s cable alone does not guarantee a 40 Gb/s storage link: the enclosure, controller, cable, filesystem, thermal state, and negotiated protocol must all support it. A 10 Gb/s enclosure remains limited to 10 Gb/s even on a 40 Gb/s cable.

When the new cable arrives:

1. Stop inference and backtests; unmount the volume normally.
2. Connect the enclosure directly without a hub.
3. Confirm whether it enumerates under Thunderbolt/USB4 or USB and record negotiated speed.
4. Run `./scripts/storage-benchmark.sh 16` twice, retaining both JSON results.
5. Compare sustained read/write rates and enclosure temperature.
6. Do not change model profiles until the new path is stable across sleep/wake and a multi-hour workload.
