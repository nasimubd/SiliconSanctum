# Silicon Sanctum

Silicon Sanctum is a Rust control plane for reproducible local AI inference and
quantitative-development workflows on Apple Silicon. It keeps model assets on
external NVMe, selects a model according to measured hardware capacity, and
exposes one local service that applications and coding agents can use through
standard APIs.

## About

Silicon Sanctum is aimed at high-frequency AI inference in the practical sense:
many small, latency-sensitive decisions around an agent or research workflow,
with predictable local execution and minimal network dependence. It is not a
claim of financial high-frequency trading performance and it does not place
orders.

The architecture separates three jobs:

```text
Rust control plane
  ├─ hardware benchmark and model-fit recommendation
  ├─ OpenAI-compatible /v1 API
  ├─ Anthropic-compatible /v1/messages API
  ├─ agent adapters and automatic local environment setup
  ├─ decision plane: typed routing, confidence, policy, and verification
  └─ model backends: MLX, llama.cpp, Ollama, and future native backends
```

The current release ships the single-binary API server, hardware probe,
benchmark command, Ollama supervision, and agent launch adapters described in
[`docs/CONTROL_PLANE.md`](docs/CONTROL_PLANE.md). Native MLX/llama.cpp workers,
deep quality benchmarking, and the Kimi backend remain future work.

## What it solves today

- Reproducible local model profiles and manifests.
- External-NVMe storage for model weights, caches, and benchmark artifacts.
- Ollama, llama.cpp, and MLX process supervision on Apple Silicon.
- Context and memory guardrails for small local coding models.
- Repository-aware agent workflows through Aider and Claude Code.
- Deterministic quantitative research and backtest orchestration with safety
  checks, manifests, seeds, and no-live-order defaults.
- A Rust foundation for routing, speculative execution, prefix caching,
  chunked prefill, pressure handling, and typed decision policies.

It does not currently make arbitrary trillion-parameter models fit in RAM. The
Kimi K3 backend is documented as a future native backend and is explicitly
**coming soon**, not included in this release. See
[`docs/KIMI_BACKEND.md`](docs/KIMI_BACKEND.md).

## Current status

The tested workstation is an M1 Pro Mac with 16 GiB unified memory. Existing
profiles include Qwen3.5 4B, Qwen3.5 9B, an experimental Qwen3.5 long-context
profile, and a guarded Qwen3.8 27B profile. The 1M profile is an extrapolation
experiment, not a verified native-million-token deployment; it is guarded at
64 GiB RAM. See [`docs/LONG_CONTEXT.md`](docs/LONG_CONTEXT.md).

The Rust runtime already contains backend-neutral registry, supervisor, routing,
gateway, speculative, context, and decision contracts. Several contracts are
tested as foundations; the shipped binary currently uses Ollama as its
managed inference worker and exposes the compatibility gateway around it.

## Installation and intended user experience

The distribution target is Homebrew. After tapping the published Silicon
Sanctum repository, install the released binary with:

```bash
brew tap nasimubd/silicon-sanctum
brew install silicon-sanctum
```

The installed binary exposes one generic server command:

```bash
sanctum serve
```

The server starts or reuses Ollama, then prints its endpoint and selected
upstream model:

```text
OpenAI API:    http://127.0.0.1:8080/v1
Anthropic API: http://127.0.0.1:8080/v1/messages
Model:         <selected profile>
```

The dedicated commands are:

```bash
sanctum-serve       # generic local server
sanctum-claude      # prepare and launch Claude Code against the local API
sanctum-codex       # prepare and launch Codex against the local API
sanctum-opencode    # prepare and launch OpenCode against the local API
sanctum-aider       # prepare and launch Aider/Aether against the local API
sanctum-doctor      # inspect installation, backend, storage, and API health
sanctum-benchmark   # rerun the reproducible hardware/model benchmark
```

The canonical executable names are lowercase. `sanctum-aider` is the Aider
adapter; an Aether adapter will use the same command shape once Aether's public
integration contract is verified. The agent commands reuse a healthy local
server or start one on an available local port, export only process-scoped
provider variables, launch the installed client, and stop a server they start.
They do not overwrite global credentials.

The installation benchmark is intentionally conservative: it reports host
capacity, backend reachability, model availability, and a generation probe.
`docs/BENCHMARKING.md` defines the deeper repeatability, memory, thermal, and
quality gates that will extend this fast installation check.

## Current commands

```bash
cp .env.example .env       # set the exact external NVMe volume name
./scripts/bootstrap.sh
local-ai doctor
local-ai serve daily
local-ai status
local-ai agent claude qwen35-4b-coding
local-ai benchmark qwen35-4b-coding 32768
local-ai stop
```

The current implementation uses Ollama as the managed server. `scripts/serve.sh`
can alternatively launch llama.cpp with a pinned GGUF. Model files and runtime
state live below `AI_ROOT`, normally on the external volume.

## Hardware recommendations

Installation must distinguish model *fit* from model *quality*. The benchmark
plan defines two recommendations:

1. **Comfortable**: fits physical memory without swap and meets the selected
   interactive latency target under a representative prompt.
2. **Robust but delayed**: fits and completes correctly, but cold load, prompt
   processing, or token latency is visibly slower.

Recommendations must be based on measured memory headroom, storage bandwidth,
prompt processing, decode rate, first-token latency, context growth, and thermal
repeatability—not parameter count alone. See
[`docs/BENCHMARKING.md`](docs/BENCHMARKING.md).

## Decision models and Jev

The decision plane is where Silicon Sanctum can become faster and more reliable
without pretending a small generative model is a frontier model. A decision
model can classify intent, select a backend, decide whether a tool call is
safe, score confidence, detect ambiguity, and gate escalation. The final policy
and side effects remain in Rust.

TypeSafe's Jev is a relevant hosted System One decision model. Based on the
official TypeSafe documentation checked on 2026-09-25, Jev is accessed through
TypeSafe's API; its weights are not published as an open-weight local model.
TypeSafe's SDKs and `system-one-adapter` are open source, but they do not make
Jev locally runnable. See [`docs/DECISION_MODELS.md`](docs/DECISION_MODELS.md).

## Safety boundary

This is research infrastructure, not trading advice. Do not connect it to live
order execution without independent controls and explicit human authorization.
Local model output is not an authority: tests, deterministic tools, manifests,
and independent verification remain authoritative.

## Further documentation

- [`docs/CONTROL_PLANE.md`](docs/CONTROL_PLANE.md) — unified binary, APIs, and
  agent commands.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — ordered implementation plan, current
  state, and acceptance criteria.
- [`docs/BENCHMARKING.md`](docs/BENCHMARKING.md) — reproducible hardware and
  model-fit benchmark plan.
- [`docs/DECISION_MODELS.md`](docs/DECISION_MODELS.md) — Jev status and the
  local decision-plane strategy.
- [`docs/KIMI_BACKEND.md`](docs/KIMI_BACKEND.md) — Kimi K3 backend plan,
  explicitly coming soon.
- [`docs/LONG_CONTEXT.md`](docs/LONG_CONTEXT.md) — current 1M-context limits.
- [`docs/PHASE1_DARWIN_CORE.md`](docs/PHASE1_DARWIN_CORE.md) — Darwin storage
  and scheduling foundations.

## License

MIT. Model weights, provider services, and third-party tools retain their own
licenses and terms.

## Citation

```bibtex
@software{silicon_sanctum,
  title = {Silicon Sanctum: Reproducible local inference and quantitative-development workstation},
  author = {MD NASIM},
  url = {https://github.com/nasimubd/SiliconSanctum}
}
```
