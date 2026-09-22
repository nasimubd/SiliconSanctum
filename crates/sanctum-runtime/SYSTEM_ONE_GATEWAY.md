# System One Gateway Guardrails

The gateway inspects prompts before invoking the non-autoregressive decision
engine. Multi-hop arithmetic, explicit counting, and relative temporal
comparisons bypass System One so jagged reasoning is never treated as a smooth
classification problem.

Arithmetic and counting requests route to deterministic interpreters. Relative
temporal comparisons route to the heavy reasoning model unless a dedicated
calendar interpreter is configured. Simple prompts continue to the concurrent
intent, tooling, and complexity fan-out router.

Every bypass includes structured evidence and a reason. Bounded event history
and per-path metrics make escalation behavior observable without retaining
unbounded prompt data.
