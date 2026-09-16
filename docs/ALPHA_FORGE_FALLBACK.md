# Alpha-forge local fallback

`msys-alpha-forage` is the research hot path. Its Mac-local jobs read the
discovery split and microstructure datasets from TickArchive; the sealed
lockbox, live ingestion, and order execution remain outside this workstation
command. This is intentional: the local AI agent can propose and run a
reproducible research experiment, but cannot turn a research request into an
order.

## Workspace map

| Repository | Local-workstation role |
|---|---|
| `msys-alpha-forage` | Alpha discovery, local discovery-split and tick-archive backtests |
| `msys-dataforge` | Data contracts and bar construction; source of the shared value-bar builder |
| `msys-alpha-execution` | Separate paper/production execution boundary; never invoked by `local-ai backtest` |
| `indicatorbench` | Indicator research and test fixtures; useful as a secondary evaluation corpus |
| `binance-mechanics-sim` | Exchange-mechanics simulation; use to challenge fill/cost assumptions |

Other repositories in the workspace are not automatically granted to the
backtest runner. Register a project only after declaring its data inputs,
research-only commands, and artifact rules in `config/backtests.json`.

## Current local jobs

`local-ai backtest list` exposes six allowlisted alpha-forge jobs. Each run:

1. requires TickArchive data roots to exist;
2. refuses a dirty research checkout unless `--allow-dirty` is explicit;
3. refuses to run while a local inference model is resident on the 16 GiB Mac;
4. stores command, Git SHA, data roots, timestamps, exit status, output log,
   and a run manifest under `$AI_ROOT/benchmarks/backtests`.

The initial smoke result on 2026-09-16 was
`fade-comparison` at alpha-forge commit `8e4c24f`: both strategies were
net-negative after the specified costs. This is a validation result, not a
trading signal.

## State-of-the-art workstation design

The practical design is a tool-first local agent, not a single huge model:

- **Inference:** use Ollama's MLX-backed Apple-Silicon path for the small
  local coding model. Keep inference and backtesting mutually exclusive on
  this 16 GiB machine. Ollama documents its MLX runtime as its Apple-Silicon
  performance path.
- **Agent:** Qwen Code is the lean terminal fallback; OpenHands is the
  stronger sandboxed-agent option when a larger machine or remote model is
  available. Both can target an OpenAI-compatible local endpoint.
- **Repository context:** assemble a repository map, lexical search results,
  symbol/AST references, Git history, and ranked snippets. Do not fill the
  model context with Parquet or raw tick files. Repository-level retrieval
  research supports multi-stage retrieval and reranking over a raw full dump.
- **Data engine:** retain Polars for columnar transforms and DuckDB for
  repeated analytical SQL. DuckDB recommends Parquet row groups of 100K–1M
  rows and files around 100 MB–10 GB; its NVMe/SSD guidance directly fits the
  measured ~1 GB/s TickArchive path.
- **Verifier:** add a second agent pass that only checks point-in-time joins,
  look-ahead, splits/embargoes, fill assumptions, fees, and manifests. Tests
  and the backtest output—not model prose—are the authority.

Sources: [Ollama MLX runtime](https://ollama.com/blog/mlx),
[Qwen Code](https://github.com/QwenLM/qwen-code),
[OpenHands local-LLM guide](https://github.com/OpenHands/docs/blob/main/openhands/usage/llms/local-llms.mdx),
[DuckDB Parquet guidance](https://duckdb.org/docs/current/guides/performance/file_formats),
and [repository-level retrieval research](https://arxiv.org/abs/2502.07067).

## Next high-value additions

1. Add a read-only DuckDB catalog over each approved TickArchive dataset and
   expose schema/sample/statistics commands to the agent.
2. Register `indicatorbench` and `binance-mechanics-sim` only after their
   local research commands are audited for data and execution boundaries.
3. Add a backtest-result parser that writes a compact metrics row alongside
   each manifest, enabling the agent to compare runs without re-reading logs.
4. Build a hybrid code index (ripgrep/BM25 plus embeddings and reranking) over
   the approved repositories; this is more useful on 16 GiB than a nominal 1M
   active context.
