<div align="center">

<img src="assets/silicon-sanctum.svg" alt="Silicon Sanctum" width="128">

# Silicon Sanctum

### A local inference control plane for fast, private AI workflows

Run one Rust binary. Keep model traffic on your machine. Give Claude Code,
Codex, OpenCode, Aider, and ordinary applications standard OpenAI or Anthropic
endpoints.

[![Release](https://img.shields.io/github/v/release/nasimubd/SiliconSanctum?display_name=tag&sort=semver&color=7c3aed)](https://github.com/nasimubd/SiliconSanctum/releases)
[![Rust CI](https://img.shields.io/github/actions/workflow/status/nasimubd/SiliconSanctum/rust-core.yml?label=Rust%20CI)](https://github.com/nasimubd/SiliconSanctum/actions/workflows/rust-core.yml)
[![Security](https://img.shields.io/github/actions/workflow/status/nasimubd/SiliconSanctum/security.yml?label=Security)](https://github.com/nasimubd/SiliconSanctum/actions/workflows/security.yml)
[![License](https://img.shields.io/github/license/nasimubd/SiliconSanctum)](LICENSE)
[![Stars](https://img.shields.io/github/stars/nasimubd/SiliconSanctum?style=social)](https://github.com/nasimubd/SiliconSanctum/stargazers)

[Install](#install) · [Quick start](#quick-start) · [API](#api) · [Commands](#dedicated-commands) · [Docs](#documentation)

</div>

---

## Why Silicon Sanctum?

Most local model tools make you choose between a model runner, an API gateway,
an agent configuration, and a hardware guess. Silicon Sanctum provides the
small control plane between those pieces:

```text
your app / coding agent
          │ OpenAI or Anthropic protocol
          ▼
   sanctum (Rust binary)
   ├─ hardware probe + model-fit report
   ├─ local API gateway + streaming SSE
   ├─ scoped Claude/Codex/OpenCode/Aider launchers
   ├─ Ollama lifecycle and health supervision
   └─ decision, routing, memory, and performance foundations
          │
          ▼
   local model backend (Ollama today; native backends later)
```

The result is a private local endpoint that existing AI clients already know
how to use—without making an open-weight model magically equivalent to a
frontier model. Quality and latency are measured per model and per machine.

## Highlights

- **One binary:** `sanctum` plus canonical command aliases.
- **Two compatible protocols:** OpenAI-compatible `/v1` routes and Anthropic
  Messages at `/v1/messages`.
- **Agent-ready:** launch Claude Code, Codex, OpenCode, or Aider with scoped
  environment variables and no global credential mutation.
- **Hardware-aware:** report comfortable models separately from models that can
  run with noticeable delay.
- **Private by default:** local traffic goes to a local backend; no provider
  credentials are required for the gateway.
- **Apple Silicon first:** released binaries target macOS ARM64 and Intel, with
  Docker and source builds available for other environments.
- **Honest boundaries:** Kimi K3 is documented as a future backend and is not
  exposed as supported today.

## Install

Choose the path that fits your workflow. The current release is `v1.15.2`.

### Homebrew (recommended)

```bash
brew tap nasimubd/silicon-sanctum
brew install silicon-sanctum
```

Upgrade later with:

```bash
brew update
brew upgrade silicon-sanctum
```

### Prebuilt release installer

The installer detects Apple Silicon or Intel macOS, downloads the matching
released archive, verifies its SHA-256 checksum, and installs into
`~/.local/bin` without requiring `sudo`:

```bash
curl -fsSL https://raw.githubusercontent.com/nasimubd/SiliconSanctum/main/install.sh | sh
```

Pin an explicit release or installation directory when desired:

```bash
SANCTUM_VERSION=v1.15.2 \
SANCTUM_INSTALL_DIR="$HOME/.local/bin" \
  sh -c 'curl -fsSL https://raw.githubusercontent.com/nasimubd/SiliconSanctum/main/install.sh | sh'
```

### Docker

The image runs the gateway in a container and connects to an Ollama service on
the host. Build it locally so the image is always tied to the source you
reviewed:

```bash
docker build -t silicon-sanctum:local .
docker run --rm --add-host=host.docker.internal:host-gateway \
  -p 8080:8080 \
  silicon-sanctum:local
```

Set `SANCTUM_UPSTREAM` when the model backend is elsewhere. The container does
not bundle model weights or silently download them.

### Build from source

Requires Rust 1.90 or newer:

```bash
git clone https://github.com/nasimubd/SiliconSanctum.git
cd SiliconSanctum
cargo install --path crates/sanctum-runtime --bin sanctum
```

For development without installing:

```bash
cargo run --release -p sanctum-runtime --bin sanctum -- serve
```

The prebuilt release installer currently supports macOS. Docker and source
builds are the supported paths for Linux and other environments; hardware
acceleration and backend availability remain platform-dependent.

## Quick start

Make sure Ollama is installed and has at least one compatible model, then run:

```bash
sanctum-doctor       # inspect hardware, backend, storage, and model fit
sanctum-benchmark    # run the reproducible smoke benchmark
sanctum-serve        # start the local gateway
```

The server prints both endpoint families:

```text
OpenAI API:    http://127.0.0.1:8080/v1
Anthropic API: http://127.0.0.1:8080/v1/messages
```

Useful environment variables:

```bash
SANCTUM_HOST=127.0.0.1
SANCTUM_PORT=8080
SANCTUM_UPSTREAM=http://127.0.0.1:11434
SANCTUM_MODEL=qwen3.5:4b-q4_K_M
```

If port 8080 is already occupied, the dedicated agent launchers select a free
local port automatically. `sanctum-serve` itself reports a bind error so an
explicit server configuration is never silently ignored.

## API

### OpenAI-compatible

Point any OpenAI-compatible client at `http://127.0.0.1:8080/v1` and use any
local model returned by `/v1/models`:

```bash
curl http://127.0.0.1:8080/v1/chat/completions \
  -H 'content-type: application/json' \
  -d '{
    "model": "qwen3.5:4b-q4_K_M",
    "messages": [{"role": "user", "content": "Reply with OK."}],
    "stream": false
  }'
```

Available OpenAI-shaped routes:

| Route | Purpose |
| --- | --- |
| `GET /v1/models` | Discover backend models |
| `POST /v1/chat/completions` | Chat generation and SSE streaming |
| `POST /v1/completions` | Completion-shaped compatibility route |
| `POST /v1/responses` | Responses-shaped compatibility route |

### Anthropic-compatible

Use the same server with clients that speak the Anthropic Messages API:

```bash
curl http://127.0.0.1:8080/v1/messages \
  -H 'content-type: application/json' \
  -H 'x-api-key: local' \
  -d '{
    "model": "qwen3.5:4b-q4_K_M",
    "max_tokens": 128,
    "messages": [{"role": "user", "content": "Reply with OK."}]
  }'
```

The gateway translates system prompts, messages, non-streaming responses, and
streaming SSE events to and from the local backend.

## Dedicated commands

```text
sanctum-serve       Start the generic local server
sanctum-claude      Start/reuse the server and launch Claude Code
sanctum-codex       Start/reuse the server and launch Codex
sanctum-opencode    Start/reuse the server and launch OpenCode
sanctum-aider       Start/reuse the server and launch Aider
sanctum-doctor      Inspect installation, backend, storage, and API health
sanctum-benchmark   Rerun the reproducible hardware/model benchmark
```

The Aider-shaped command is also the reserved compatibility shape for Aether;
an Aether-specific adapter will land once its public integration contract is
stable. Missing client executables fail with an actionable error after the
server readiness check.

## Hardware fit and model recommendations

`sanctum-doctor` is the fast installation check. `sanctum-benchmark` adds a
generation probe and reports backend reachability, model availability,
generation time, output tokens, and tokens per second when the probe completes.

Recommendations are deliberately split:

- **Comfortable:** the model fits the detected memory class and is the primary
  local recommendation.
- **Robust but delayed:** the model is expected to run, but cold load, prompt
  processing, or decode latency may be noticeable.

The current laptop result is Qwen3.5 4B as comfortable, with Qwen3.5 9B and
Qwen3.8 27B in the delayed tier. These are machine-specific recommendations,
not universal performance claims. See [`docs/BENCHMARKING.md`](docs/BENCHMARKING.md)
for the deeper benchmark design based on llama.cpp, MLX-LM, vLLM, SGLang, and
lm-evaluation-harness.

## What is supported today?

| Capability | Status |
| --- | --- |
| Rust single-binary CLI/server | Supported |
| Ollama supervision | Supported |
| OpenAI-compatible gateway | Supported |
| Anthropic-compatible gateway | Supported |
| Claude/Codex/OpenCode/Aider launchers | Supported |
| Hardware probe and smoke benchmark | Supported |
| Native MLX or llama.cpp worker inside Sanctum | Planned |
| Local FOSS decision-model backend | Contract and research plan; implementation pending |
| Kimi K3 backend | **Coming soon; not included** |

## Architecture and roadmap

The Rust decision plane is designed to route, verify, abstain, gate tools, and
escalate without granting a model authority over side effects. TypeSafe Jev is
hosted and has no verified open-weight checkpoint; Silicon Sanctum therefore
does not claim to run Jev locally. Candidate local FOSS runtimes are ONNX
Runtime, Candle, and Apple-native model paths, selected by calibration and
latency benchmarks rather than branding.

The separate Kimi K3 plan is documented in
[`docs/KIMI_BACKEND.md`](docs/KIMI_BACKEND.md). It remains explicitly
documentation-only until storage, memory, latency, parity, and recovery gates
pass.

## Documentation

- [`docs/CONTROL_PLANE.md`](docs/CONTROL_PLANE.md) — API and agent contract.
- [`docs/BENCHMARKING.md`](docs/BENCHMARKING.md) — hardware and model-fit methodology.
- [`docs/DECISION_MODELS.md`](docs/DECISION_MODELS.md) — Jev findings and local decision plane.
- [`docs/KIMI_BACKEND.md`](docs/KIMI_BACKEND.md) — future Kimi integration.
- [`docs/LONG_CONTEXT.md`](docs/LONG_CONTEXT.md) — current million-token limits.
- [`docs/ROADMAP.md`](docs/ROADMAP.md) — implementation status and next milestones.

## Contributing

Issues and pull requests are welcome. Run the local gates before submitting:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
make validate
./scripts/security-scan.sh
```

Use Conventional Commits. Please include reproducible hardware, model, backend,
and latency details for performance changes.

## Safety and limitations

Silicon Sanctum is local inference infrastructure, not trading advice and not a
frontier-model replacement. Do not connect generated output to live financial
execution without independent controls and explicit human authorization.

## License

MIT. Model weights, provider services, and third-party tools retain their own
licenses and terms.

<div align="center">

Built for private, measurable, local inference.

</div>
