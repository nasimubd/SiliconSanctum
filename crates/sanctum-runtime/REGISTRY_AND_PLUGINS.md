# Model Registry and Plugin Boundary

The model registry keeps runtime configuration independent from individual
inference backends. Every entry declares its backend, weight format,
quantization, fixed weight allocation, per-token KV cost, and a strictly
increasing context ladder.

Context planning walks the ladder from largest to smallest and selects the
largest level whose weights and KV cache fit the live arbiter budget. Invalid,
duplicate, or unknown model identifiers fail before process launch.

External domain features connect through plugin manifests. A manifest contains
only an identifier, an HTTP or gRPC endpoint, and advertised capabilities.
Requests and responses use opaque byte payloads so the core runtime has no
dependency on quantitative analysis, backtesting, market data, or any other
domain-specific schema.

Transport implementations conform to the common plugin transport trait. This
keeps discovery and routing stable while allowing HTTP and gRPC clients to be
implemented independently.
