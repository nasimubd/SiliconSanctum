# Unified Rust control plane

## Product goal

After installation, users should be able to run one binary and one command:

```bash
sanctum serve
```

The binary should own discovery, benchmark selection, backend startup, health
checks, API serving, model lifecycle, and graceful shutdown. Shell scripts and a
persistent Ollama LaunchAgent may remain compatibility implementations during
the migration, but they should not be required for the final path.

## API contract

Expose both protocols from the same listener:

```text
GET  /v1/models
POST /v1/chat/completions
POST /v1/responses
POST /v1/completions
POST /v1/messages
```

The OpenAI surface should support streaming, tool calls, structured output,
usage accounting, cancellation, and model discovery. The Anthropic surface
should support the Messages request shape, system prompts, tools, streaming
SSE, stop reasons, and content blocks. Internally both should normalize into a
backend-neutral request and response model; protocol-specific rendering belongs
at the edge.

`llama-server` is a useful FOSS compatibility reference because it documents
both OpenAI-compatible endpoints and an Anthropic Messages-compatible endpoint.
The compatibility suite should compare requests, streaming events, tool calls,
errors, cancellation, and token accounting against captured fixtures.

## Dedicated agent commands

The target commands are:

```bash
sanctum claude
sanctum codex
sanctum opencode
sanctum aether
```

Each command should:

1. Start or reuse the local server.
2. Select a compatible model and context budget.
3. Export the minimum documented environment variables or generate a scoped
   temporary configuration.
4. Verify the target client is reachable before handing over control.
5. Launch the client when the executable is installed, otherwise print one
   actionable status message and the already-running endpoint.
6. Never replace credentials, global configuration, or an existing remote
   provider silently.

Claude Code and Codex should use the Anthropic and OpenAI-compatible surfaces
respectively where their current client contracts permit. OpenCode and Aether
should use provider adapters selected from their supported environment/config
interfaces. Because these clients can change, each adapter needs a versioned
smoke test and a capability report rather than undocumented configuration hacks.

## Runtime shape

The existing Rust modules map naturally to the control plane:

```text
CLI
 ├─ benchmark + recommender
 ├─ supervisor + backend registry
 ├─ protocol gateway (OpenAI / Anthropic)
 ├─ decision/router policy
 └─ model worker (MLX / llama.cpp / Ollama / future native backend)
```

The first implementation milestone should add a real binary entry point,
HTTP/SSE handling, normalized request types, backend capability negotiation,
and end-to-end tests. Do not claim frontier equivalence from API compatibility:
quality, latency, tool reliability, and refusal behavior must be measured per
profile.
