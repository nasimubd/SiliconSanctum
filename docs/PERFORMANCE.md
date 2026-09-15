# Performance results

Measurements on the M1 Pro 16 GB Mac, with model files on `/Volumes/TickArchive`.

| Backend/profile | Context | Cold activation | First TTFT | Cached TTFT | Decode |
|---|---:|---:|---:|---:|---:|
| Ollama Qwen3.5 4B chat | 8,192 | 86.4 s | 0.903 s | 0.082 s | 32.2 tok/s |
| Ollama Qwen3.5 4B coding | 32,768 | 85.9 s | 0.886 s | 0.083 s | 32.2 tok/s |
| Ollama Qwen3.5 9B focused | 32,768 | 152.2 s | 1.571 s | 0.131 s | 20.1 tok/s |
| MLX Qwen3.5 4B 4-bit | native | 70.9 s | — | — | 52.1 tok/s |
| llama.cpp text-only Q4_K_M | 8,192 | 69.2 s | — | — | 33.0 tok/s |

The persistent LaunchAgent and unlimited keep-alive eliminate repeated daemon startup. Prompt-cache reuse reduces the measured 290-token prompt from about 0.9 s to 0.08 s. The remaining cold cost is external-storage loading; re-run `make benchmark` after the 40 Gbps cable arrives.

```bash
make benchmark
./scripts/performance-benchmark.sh qwen35-4b-coding 32768
```

The 1M profile remains experimental: Qwen3.5 supports 262K natively, while 1,010,000 is extrapolated and consumes substantial KV memory.
