# Long-context and local-agent strategy

## Current decision

The M1 Pro in this workstation has 16 GiB unified memory. It can use the
existing 4B model for local coding and constrained backtest work, but it is
not a credible host for a useful 1M-token coding agent. The existing
`qwen35-4b-1m` profile is retained as a *guarded experiment*: it now refuses
to start below 64 GiB RAM and its 1,010,000-token setting is extrapolated from
262K native support.

For a verified open-weight 1M context experiment, use
`Qwen2.5-7B-Instruct-1M` or `Qwen2.5-14B-Instruct-1M` with Qwen's sparse-
attention vLLM framework on a suitably provisioned CUDA host. Qwen released
these checkpoints specifically for 1M-token contexts; do not substitute an
ordinary 128K checkpoint and merely raise `num_ctx`.

The strongest open-weight *coding* candidate is Qwen3-Coder-480B-A35B:
it is coding/agent focused, has 256K native context, and supports a 1M
extension through YaRN. It is not a local-Mac option: its 480B total
parameters require a multi-GPU or large remote deployment. Llama 4 Scout is
an alternative with a 10M advertised context, but its 109B total parameters
and license make it equally unsuitable for this machine. No model should be
described as a 1M local setup unless it passes the context ladder and
needle/retrieval tests on the target hardware.

Sources: [Qwen2.5-1M announcement](https://qwenlm.github.io/blog/qwen2.5-1m/),
[Qwen3-Coder model card](https://huggingface.co/Qwen/Qwen3-Coder-480B-A35B-Instruct),
and [Llama 4 model card](https://github.com/meta-llama/llama-models/blob/main/models/llama4/MODEL_CARD.md).

## Making the local setup useful for backtesting

Treat the local model as a private, tool-using research assistant, not as a
replacement for independent validation. The near-term capability target is:

1. Retrieve a compact repository map plus relevant functions with hybrid
   keyword/vector search and reranking; use the 1M window only for audits.
2. Give the agent narrow tools: read/search, edit a branch, run a pinned
   backtest, inspect its manifest, and run tests. Require a proposed plan and
   diff before execution.
3. Make every run deterministic: pinned tick-data manifest, code commit,
   parameters, random seed, calendar/timezone, costs/slippage assumptions, and
   result artifact.
4. Enforce leakage checks mechanically: point-in-time data, walk-forward
   splits, embargo/purging where labels overlap, and a final untouched holdout.
5. Use a separate verifier pass to challenge timestamps, corporate actions,
   survivorship, look-ahead, fills, turnover, and sensitivity to fees.

This arrangement can replace Claude Code for private, bounded implementation
and repeatable backtest orchestration. It does not yet match a frontier hosted
agent for ambiguous multi-file design, deep quantitative reasoning, or
reliable 1M-token analysis. The highest-leverage upgrade is the evaluation and
tool interface, not simply a larger context window: SWE-agent shows that a
purpose-built agent-computer interface improves repository navigation and test
execution, while repository-level retrieval research supports multi-stage
reranking over a raw full-context dump.

Further reading: [SWE-agent](https://arxiv.org/abs/2405.15793),
[SWE-bench](https://arxiv.org/abs/2310.06770), and
[repository-level code retrieval](https://arxiv.org/abs/2502.07067).
