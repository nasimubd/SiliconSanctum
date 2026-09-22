# Radix Prefix Cache

The prefix cache stores token sequences in a radix tree and returns the longest
shared prefix for multi-turn sessions. Terminal nodes carry stable cache
handles, byte accounting, and recency metadata. Session bindings can be
extended as conversations grow.

Capacity limits bound node and byte growth. Cache statistics record entries,
bytes, hits, and misses, while bounded events expose insertion, lookup, session,
and eviction activity.

Prefix reuse is restricted to pure full-attention models. Sliding-window,
recurrent, and hybrid architectures are rejected before cache activation,
including Gemma sliding-window and Qwen SSM families that cannot safely slice
MLX prefix state.
