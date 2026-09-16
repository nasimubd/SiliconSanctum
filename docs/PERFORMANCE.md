# Performance results

Measurements on the M1 Pro 16 GB Mac, with model files under `AI_ROOT`.

| Backend/profile | Context | Cold activation | First TTFT | Cached TTFT | Decode |
|---|---:|---:|---:|---:|---:|
| Ollama Qwen3.5 4B chat | 8,192 | 86.4 s | 0.903 s | 0.082 s | 32.2 tok/s |
| Ollama Qwen3.5 4B coding (2026-09-16) | 32,768 | 6.045 s | 0.892 s | 0.084 s | 32.0 tok/s |
| Ollama Qwen3.5 9B focused | 32,768 | 152.2 s | 1.571 s | 0.131 s | 20.1 tok/s |
| MLX Qwen3.5 4B 4-bit | native | 70.9 s | — | — | 52.1 tok/s |
| llama.cpp text-only Q4_K_M | 8,192 | 69.2 s | — | — | 33.0 tok/s |

The persistent LaunchAgent and unlimited keep-alive eliminate repeated daemon startup. Prompt-cache reuse reduces the measured 290-token prompt from about 0.9 s to 0.08 s. The remaining cold cost is external-storage loading.

## TickArchive storage benchmark — 2026-09-16

After the cable change, TickArchive remains a USB 3.1 / up-to-10-Gb/s device,
not a Thunderbolt/USB4 device. Two 16 GiB direct passes measured 983.4 and
1001.4 MB/s write, and 977.1 and 979.1 MB/s read. This is a substantial
improvement over the prior ~40 MB/s cold-read observation. The fresh coding
profile activation completed in 6.045 s (previously 85.9 s), while first and
cached TTFT remained 0.892 s and 0.084 s. That run began with 5.77 GiB swap in
use, so it is strong storage evidence rather than a clean low-memory baseline.

```bash
make benchmark
./scripts/performance-benchmark.sh qwen35-4b-coding 32768
```

The 1M profile remains experimental and is guarded at 64 GiB RAM: Qwen3.5
supports 262K natively, while 1,010,000 is extrapolated and consumes
substantial KV memory.
