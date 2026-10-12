# Repository instructions for coding agents

Silicon Sanctum is a local AI workstation for Apple Silicon, with a Rust control
plane, macOS systems code, shell orchestration, and optional Kaggle batch jobs.
Treat this file as the shared operating guide for changes to this repository.
For product usage, start with [README.md](README.md); for security boundaries,
read [SECURITY.md](SECURITY.md).

## Find the right source of truth

- `crates/sanctum-core/` owns Darwin telemetry, memory pressure, direct I/O,
  Metal storage, and other low-level platform contracts. Read
  [docs/PHASE1_DARWIN_CORE.md](docs/PHASE1_DARWIN_CORE.md) before changing them.
- `crates/sanctum-runtime/` owns process supervision, routing, context, API
  translation, and the `sanctum` CLI. Its [README](crates/sanctum-runtime/README.md)
  and colocated design notes explain the relevant contract.
- `bin/local-ai`, `scripts/`, `config/`, and `Makefile` provide the existing
  workstation workflows. Keep their CLI behavior and configuration aligned.
- `kaggle/` is a separate, quota-bound batch worker. It is not part of the
  local inference process; see [docs/KAGGLE.md](docs/KAGGLE.md).
- Documents that describe future work, including [docs/ROADMAP.md](docs/ROADMAP.md)
  and [docs/CONTROL_PLANE.md](docs/CONTROL_PLANE.md), are plans. Confirm behavior
  in code and tests before claiming a feature is implemented.

## Work on a change

1. Read the relevant code, tests, and nearby design document. State any
   assumption that affects safety or compatibility.
2. Make a focused change. Preserve existing command names, config keys, API
   shapes, and observable error behavior unless the task explicitly changes
   their contract.
3. Add or update a meaningful test for behavior changes. For numerical or
   research results, record inputs, version or commit, parameters, seed, and
   output path so the result can be reproduced.
4. Run the smallest relevant checks, then the applicable gates below. Report
   what ran and what requires Apple Silicon, a mounted volume, credentials, or
   another external dependency.

## Validation commands

| Change area | Command |
| --- | --- |
| Rust formatting and lint | `make core-lint` |
| Rust build and tests | `make core-check && make core-test` |
| Shell, config, and Python syntax | `make validate` |
| Credential guard before a push | `./scripts/security-scan.sh` |

The Rust CI uses `cargo fmt --all -- --check`,
`cargo clippy --workspace --all-targets --locked -- -D warnings`, and
`cargo test --workspace --locked` on macOS. The fuzz targets require nightly
Rust and `cargo-fuzz`; run them when changing their parsers or arithmetic.
Native Darwin and Metal behavior needs a macOS host, and hardware-backed Metal
validation needs Apple Silicon. Do not report a non-macOS pass as hardware
validation.

## Safety and operational boundaries

- Keep the local API bound to loopback unless the change includes a reviewed
  authentication, TLS, and access-control design. Do not silently replace a
  user's provider credentials or global agent configuration.
- Never commit `.env`, secrets, private prompts, proprietary market data,
  broker configuration, model weights, logs, or generated benchmark output.
  Store local artifacts under `AI_ROOT`, outside Git. Follow [SECURITY.md](SECURITY.md)
  for disclosure and pre-publication checks.
- Backtests are research only. Do not add live order execution or upload data
  to Kaggle without explicit authorization. Preserve dataset/seed/commit
  provenance and check timestamp alignment and look-ahead leakage.
- Treat memory budgets, pressure signals, sysctl restoration, buffer lifetime,
  alignment, and short reads as correctness constraints in the Darwin core.
  Avoid weakening checked arithmetic or cleanup on error paths.
- Commands that change a host or external state, such as `bootstrap`,
  `storage-init`, `migrate-thunderbolt`, model pulls, Kaggle submission, or
  `release:full`, are not validation commands. Inspect their effects and run
  them only when the task authorizes those effects.

## Contribution conventions

- Use Conventional Commits and keep unrelated edits out of the change.
- Update user-facing docs when commands or behavior change. Do not present
  roadmap targets as shipped features.
- Releases use the `mise` and semantic-release tasks in `mise.toml`; only the
  release workflow should change `VERSION`, `CHANGELOG.md`, tags, or releases.
