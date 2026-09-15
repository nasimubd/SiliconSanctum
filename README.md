# Local AI Workstation

[![MIT License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Conventional Commits](https://img.shields.io/badge/commits-Conventional%20Commits-fe5196.svg)](https://www.conventionalcommits.org/)
[![macOS](https://img.shields.io/badge/platform-macOS-000000.svg?logo=apple)](https://www.apple.com/macos/)
[![Ollama](https://img.shields.io/badge/runtime-Ollama-000000.svg)](https://ollama.com/)
[![llama.cpp](https://img.shields.io/badge/runtime-llama.cpp-4285f4.svg)](https://github.com/ggml-org/llama.cpp)
[![MLX](https://img.shields.io/badge/runtime-MLX-555555.svg)](https://github.com/ml-explore/mlx)
[![Kaggle](https://img.shields.io/badge/burst%20compute-Kaggle-20beff.svg?logo=kaggle)](https://www.kaggle.com/)

Reproducible local inference and quantitative-development tooling for Apple
Silicon Macs with model storage on an external NVMe volume. It combines
Ollama, llama.cpp, MLX, Claude Code, Aider, and checkpointed Kaggle workers for
coding, research review, and backtest experimentation.

> **Important:** This is research infrastructure, not trading advice. Never
> connect it to live order execution without independent controls and human
> authorization.

## Quick start

### Requirements

- Apple Silicon macOS (the tested machine is an M1 Pro with 16 GB unified memory)
- Homebrew and an external volume mounted at `/Volumes/TickArchive`
- A source repository for your quantitative project
- Optional: a Kaggle account for burst jobs and Claude Code for the agent UI

### Install

```bash
git clone https://github.com/nasimubd/local-ai-workstation.git
cd local-ai-workstation
cp .env.example .env
./scripts/bootstrap.sh
local-ai doctor
```

If the command is not found in an existing terminal, open a new terminal or
run `export PATH="$HOME/.local/bin:$PATH"`.

### Start local inference

```bash
local-ai serve daily       # Qwen3.5 4B, 32K context: default coding mode
local-ai status
```

The managed Ollama LaunchAgent stays on localhost, keeps one model resident,
and stores weights/cache/benchmarks on `TickArchive`. The first model load is
storage-bound; later requests reuse memory and the prompt cache.

## Claude Code workflow

Claude Code runs on the Mac and uses Ollama's Anthropic-compatible local API;
Qwen performs inference, while Claude Code supplies the terminal/tool harness.
Your repository, tests, backtests, and Git history remain local.

```bash
cd /path/to/your/quant-project
local-ai agent claude qwen35-4b-coding
```

For a long-context audit, start progressively at 524K:

```bash
local-ai agent claude qwen35-4b-1m 524288
```

Advance to `786432` and `1010000` only after retrieval accuracy, tool calls,
memory pressure, and swap usage pass validation. Qwen3.5's native context is
262K; larger values are extrapolation experiments. Use long context for
repository-wide review, then switch back to 32K/128K for iterative edits.

## Quantitative research and backtesting

Ask the agent to inspect code, propose a diff, run tests, and execute a
deterministic backtest. Require it to record the Git commit, data manifest,
random seed, parameters, and output path. Keep raw market data and credentials
outside prompts and Git. Review every change involving timestamps, execution,
portfolio accounting, and leakage.

```text
Run the backtest with the pinned dataset and seed. Check for look-ahead,
survivorship, timestamp, and train/test leakage. Write metrics and the run
manifest to /Volumes/TickArchive/ai-workstation/benchmarks. Do not place orders.
```

## Kaggle burst compute

Kaggle is a separate, quota-bounded batch worker—not a live extension of the
Claude Code process. Configure `kaggle/job.json`, authenticate, and submit:

```bash
kaggle auth login
make kaggle-submit
kaggle kernels status <your-kaggle-username>/quant-long-context-worker
make kaggle-output
```

The worker embeds its manifest, enforces a deadline, and writes checkpoints.
Upload only approved code/data/model artifacts. Kaggle's T4×2 allocation is two
16 GB GPUs; use a tensor-parallel CUDA runtime for large models. Start long
context jobs at 524K and checkpoint each shard. Do not operate multiple accounts
to pool quota; follow [Kaggle's Terms](https://www.kaggle.com/terms).

See [Kaggle operations](docs/KAGGLE.md) and [performance results](docs/PERFORMANCE.md).

## Profiles and commands

| Command | Profile | Use |
|---|---|---|
| `local-ai serve chat` | Qwen3.5 4B / 8K | Fast conversation |
| `local-ai serve daily` | Qwen3.5 4B / 32K | Default coding and agents |
| `local-ai serve quality` | Qwen3.5 9B / 32K | Focused quality pass |
| `local-ai serve long 524288` | Qwen3.5 4B | Experimental long context |
| `local-ai serve qwen38` | Qwen3.8 27B | Guarded, memory-heavy experiment |

Useful commands: `local-ai doctor`, `local-ai validate`, `local-ai benchmark`,
`local-ai context-ladder`, `local-ai lock-models`, and `local-ai bundle`.

## Reproducibility and recovery

Configuration, scripts, manifests, and documentation are versioned in Git.
Model weights are deliberately excluded. Run `local-ai lock-models` after model
changes and `local-ai bundle` to refresh the offline recovery bundle on
`TickArchive`. See [recovery](docs/RECOVERY.md) and [Thunderbolt migration](docs/THUNDERBOLT_MIGRATION.md).

## Contributing

Use [Conventional Commits](https://www.conventionalcommits.org/), add tests or
benchmark evidence for behavior changes, and keep pull requests focused. Do
not commit credentials, proprietary market data, model weights, or generated
runtime artifacts.

## Releasing

Releases follow the same `mise` + semantic-release convention used by the
quantitative research projects in this organization. Run from a clean `main`
checkout with GitHub authentication available:

```bash
mise run release:drift       # inspect version, tags, and unreleased commits
mise run release:dry         # preview the next release; no changes made
mise run release:full        # preflight, changelog/version, tag, GitHub Release
```

`release:preflight` requires a clean tree, `main`, Conventional Commits, and a
passing validation suite. semantic-release derives the version from commit
types, updates `VERSION` and `CHANGELOG.md`, creates a `vX.Y.Z` tag, and publishes
the GitHub Release. See `release.config.cjs` and `mise-tasks/release/` for the
reproducible implementation.

## Technology credits

This project integrates [Ollama](https://github.com/ollama/ollama),
[llama.cpp](https://github.com/ggml-org/llama.cpp),
[MLX](https://github.com/ml-explore/mlx),
[MLX-LM](https://github.com/ml-explore/mlx-lm),
[Aider](https://github.com/Aider-AI/aider),
[Claude Code](https://docs.anthropic.com/en/docs/claude-code),
[Kaggle](https://www.kaggle.com/), and
[Shields.io](https://shields.io/). Their names and marks remain the property
of their respective owners.

## License

Licensed under the [MIT License](LICENSE).

## Citation

```bibtex
@software{local-ai-workstation,
  title = {local-ai-workstation: Reproducible local inference and quantitative-development workstation},
  author = {MD NASIM},
  url = {https://github.com/nasimubd/local-ai-workstation}
}
```
