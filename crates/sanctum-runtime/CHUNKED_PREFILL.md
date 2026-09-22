# Chunked Prefill Scheduler

The prefill scheduler partitions context ingestion into bounded 512-token or
1024-token chunks. It selects the smaller step for short requests and warning
memory states, and uses the larger step only when live memory has comfortable
headroom.

Plans account for both quantized KV-cache residency and full-precision
intermediate scratch memory. Scheduling fails before allocation when the host
is under critical pressure, the KV cache cannot fit in available memory, or a
chunk could exceed the configured scratch budget.

Model geometry supplies layer, KV-head, and head-dimension counts. Supported
KV widths are 4, 8, and 16 bits. Every plan exposes ordered token ranges,
estimated peak scratch bytes, and total KV bytes so an MLX integration can
initialize its cache without constructing an unbounded floating-point
scratchpad.

Cancellation tokens, bounded event history, scheduler metrics, and resumable
chunk cursors provide the control surface needed by the runtime supervisor.
