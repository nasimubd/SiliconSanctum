# Kimi K3 backend — coming soon

**Status: coming soon. The Kimi backend is not included in the current project.**

The proposed backend would integrate the separate
[`kimi-k3-in-c`](https://github.com/FareedKhan-dev/kimi-k3-in-c) engine behind
Silicon Sanctum's backend-neutral worker and API contracts. That engine is
specialized for Kimi K3's architecture and uses MXFP4 routed experts, direct
storage streaming, a packed dense trunk, and a bounded expert cache. It is not
an Ollama profile and cannot be enabled by adding a model name to
`config/models.json`.

## Planned phases

1. Validate licensing, checkpoint provenance, supported Apple Silicon paths,
   storage requirements, and reproducibility on a real target machine.
2. Define a process or library boundary so the Rust control plane can start,
   health-check, stop, and observe the C engine without copying the checkpoint
   into RAM.
3. Add a `KimiBackend` capability entry and map streaming text, cancellation,
   context limits, memory budgets, and telemetry into the common worker model.
4. Expose it through the same OpenAI and Anthropic protocol adapters.
5. Add explicit storage and RAM gating. The upstream project reports roughly
   1.56 TB for the checkpoint plus about 109 GB for its packed trunk; local
   NVMe capacity and sustained read bandwidth are therefore first-class
   requirements.
6. Verify output parity, peak RSS, token latency, cold-start behavior, and
   failure recovery before advertising the backend.

Until those gates pass, Kimi must remain documentation only. Silicon Sanctum
must not imply that the current Ollama, llama.cpp, or MLX profiles can host it.
