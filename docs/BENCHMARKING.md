# Hardware and model benchmarking plan

## Purpose

Installation should make a conservative recommendation, not guess from RAM or
parameter count. A model is **comfortable** only when it fits without swap and
passes an interactive latency budget on repeated runs. A model is **robust but
delayed** when it completes correctly but misses that latency target.

## Measurements

Record a machine and model manifest with:

- OS, chip, core counts, unified/physical memory, and thermal state.
- Model format, quantization, parameter count, context, backend, and revision.
- NVMe or volume sequential and random read throughput.
- Cold load time and resident memory.
- Prompt processing rate and time-to-first-token at 256, 2K, 8K, and the
  profile's supported context levels.
- Decode tokens/second at 128 output tokens, p50 and p95 request latency, and
  peak resident/wired memory.
- Swap/pageout activity, OOM events, errors, and throttling across a repeated
  warm run.
- A quality smoke suite: structured output, tool-call selection, code edit
  patch validity, refusal boundary, and deterministic replay where supported.

Use fixed prompts and seeds for performance comparisons, but do not use a tiny
synthetic prompt as the sole quality or capacity test. Preserve raw results and
the exact command, model digest, backend version, and Git SHA.

## Recommendation algorithm

```text
unsupported       cannot load, exceeds hard memory/storage policy, or fails smoke tests
robust/delayed     loads safely but misses interactive latency or thermal target
comfortable        safe memory headroom, stable repeated latency, smoke tests pass
```

Memory safety has priority over speed. A model that relies on swap is not
comfortable. Context recommendations must be derived from observed KV-cache
growth and a safety reserve, not from a requested `num_ctx` value alone.

## FOSS references

- [llama.cpp llama-bench](https://github.com/ggml-org/llama.cpp/tree/master/examples/llama-bench)
  — prompt/decode throughput and backend comparison.
- [llama-server](https://github.com/ggml-org/llama.cpp/tree/master/tools/server)
  — local API, slots, streaming, and server-level load tests.
- [MLX-LM benchmark](https://github.com/ml-explore/mlx-lm) — Apple Silicon
  generation and memory behavior; its long-prompt step size is relevant to
  peak-memory testing.
- [vLLM benchmarks](https://docs.vllm.ai/en/latest/cli/bench/) — latency,
  throughput, concurrency, prefix caching, and long-context workloads.
- [SGLang benchmarks](https://github.com/sgl-project/sglang/tree/main/benchmark)
  — serving, prefix caching, speculative decoding, and model-specific tests.
- [lm-evaluation-harness](https://github.com/EleutherAI/lm-evaluation-harness)
  — reproducible quality evaluation through local or OpenAI-compatible APIs.

These tools should be references and optional benchmark engines, not hidden
installation dependencies. Silicon Sanctum should provide a small native
smoke benchmark for fast installation and optionally run deeper suites later.

## Implementation stages

1. Native hardware probe and storage benchmark.
2. Backend load/health/decode benchmark with JSON manifests.
3. Memory-pressure and swap detection.
4. Repeatability and thermal run.
5. Model recommendation table with confidence and reasons.
6. Optional quality suite and user-visible benchmark report.
