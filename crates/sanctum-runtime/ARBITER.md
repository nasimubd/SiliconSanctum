# Zero-Swap Memory Arbiter

The arbiter enforces a 10.4 GiB wired-memory ceiling and preserves explicit
headroom before macOS compression or swap activity can begin.

Each sampling tick combines wired memory, available memory, observed swap use,
and the current pressure level. Swap activity, critical pressure, warning
pressure, or crossing the proactive wired threshold requests eviction. Once
wired usage falls below the recovery threshold under normal pressure, an
evicted model becomes eligible for hot reload.

Dynamic budgets reserve safety headroom first, account for model weights next,
and assign only the remaining capacity to the KV cache. All arithmetic
saturates at zero so an overloaded host cannot produce an oversized budget.

Eviction selection prefers the lowest-priority least-recently-used resident
model. Reload selection prefers the highest-priority evicted model. The bounded
event log records samples and decisions without unbounded daemon growth.
