# Silicon Sanctum execution plan

This is the implementation plan for the single-binary local inference product.
It separates work that already exists in the Rust foundation from work that
must still be built.

## Current state at a glance

| Capability | State | Evidence / next action |
|---|---|---|
| Model registry and backend metadata | Foundation exists | [`registry.rs`](../crates/sanctum-runtime/src/registry.rs); connect it to a real server |
| Process supervision | Foundation exists | [`supervisor.rs`](../crates/sanctum-runtime/src/supervisor.rs); add unified CLI ownership |
| Context and memory planning | Foundation exists | context/prefill/arbiter modules; validate against live backend telemetry |
| Prompt routing | Foundation exists | [`router.rs`](../crates/sanctum-runtime/src/router.rs); replace fixed evaluator with measured signals |
| Safety gateway | Foundation exists | [`gateway.rs`](../crates/sanctum-runtime/src/gateway.rs); keep Rust policy authoritative |
| Speculative decoding | Harness exists | draft/target contract and tests; connect to real backends |
| Prefix/radix cache | Foundation exists | radix module and tests; add cache metrics and eviction validation |
| Decision-model executor | Contract only | [`decision.rs`](../crates/sanctum-runtime/src/decision.rs) currently has a fixture executor |
| OpenAI API | Not implemented in Rust binary | implement normalized request/response and SSE layer |
| Anthropic API | Not implemented in Rust binary | implement `/v1/messages` and content-block streaming |
| Hardware benchmark installer | Shell benchmarks exist | make a native, persisted benchmark manifest and recommender |
| Agent commands | Existing Claude/Aider shell path | add versioned adapters for Claude, Codex, OpenCode, and Aether |
| Kimi backend | Not implemented | documentation-only plan; see [`KIMI_BACKEND.md`](KIMI_BACKEND.md) |

## Phase 1 — one binary and one local server

Deliver a native `sanctum` binary with:

```text
sanctum serve
  ├─ hardware/storage probe
  ├─ benchmark or load cached benchmark manifest
  ├─ choose a model/backend/context profile
  ├─ start one worker and supervise its lifecycle
  ├─ serve OpenAI and Anthropic compatibility routes
  └─ print endpoint, model, fit class, and health
```

Acceptance criteria:

- One command works from a clean installation after model assets are available.
- `/v1/models`, `/v1/chat/completions`, `/v1/responses`, and `/v1/messages`
  have streaming and non-streaming contract tests.
- Backend crashes, cancellation, malformed requests, and context exhaustion
  produce typed errors and never corrupt session state.
- Only one model is resident unless a profile explicitly allows otherwise.
- Existing Ollama/llama.cpp/MLX paths remain available during migration.

## Phase 2 — hardware-aware installation

Implement the benchmark and recommender in [`BENCHMARKING.md`](BENCHMARKING.md).
The installer should run a short native test first, then offer an optional deep
test. It must report why a model is comfortable, delayed, or unsupported.

Acceptance criteria:

- Results include hardware identity, backend version, model digest, context,
  cold load, TTFT, prompt rate, decode rate, peak memory, swap, and errors.
- A model depending on swap cannot be labelled comfortable.
- Recommendations are reproducible from a committed JSON manifest.
- The benchmark survives repeated runs and records thermal degradation.

## Phase 3 — agent adapters

Implement `sanctum claude`, `sanctum codex`, `sanctum opencode`, and
`sanctum aether` as narrow adapters. Each adapter gets a versioned capability
matrix and smoke test. The adapter must reuse a running server, configure only
its own scoped environment, avoid credential mutation, and restore state on
exit. If a client is absent, the command should still start the API and print
the exact endpoint configuration.

The target is transparent workflow behavior, not an unsupported promise that a
local model has frontier-model quality. Quality, tool-call reliability,
latency, and refusal behavior remain benchmarked per model profile.

## Phase 4 — decision plane

The decision plane sits before and after generation:

```text
request → intent/tool/risk decision → fast or strong backend
response → schema/tool/safety decision → commit, retry, review, or escalate
```

First ship deterministic Rust policies and a real local small model behind the
existing decision contract. Then add confidence calibration, abstention,
disagreement logging, and shadow evaluation against a stronger model or Jev.

The local FOSS implementation should initially use one of these supported
paths, selected by an actual benchmark on the target machine:

- Core ML for Apple-native small classifiers;
- ONNX Runtime for portable pre-exported models;
- Candle for a pure-Rust model path where supported.

Do not call any of these “Jev locally.” They are implementation options for a
local decision plane, not an open-source reproduction of TypeSafe's model.

## Phase 5 — performance and robustness

Apply the existing concepts in this order:

1. Prefix/radix cache for repeated system prompts and repository context.
2. Chunked prefill to bound memory spikes on long prompts.
3. Continuous request scheduling and one-request-per-profile defaults.
4. Speculative decoding with acceptance telemetry and safe target fallback.
5. Decision-based routing and tool-call gating with abstention.
6. Backend-specific thermal, memory-pressure, and storage feedback.

Every optimization must preserve cancellation, deterministic session state,
structured output correctness, and a slower safe fallback.

## Phase 6 — Kimi K3

Kimi remains **coming soon**. After Phase 1, implement the separate backend
according to [`KIMI_BACKEND.md`](KIMI_BACKEND.md). It must pass storage gating,
output parity, memory, latency, and recovery tests before appearing in the
model recommender.
