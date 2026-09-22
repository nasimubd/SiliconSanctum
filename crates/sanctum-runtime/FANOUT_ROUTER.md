# Speculative Fan-Out Router

The router evaluates intent, tooling necessity, and complexity concurrently.
Each axis produces a calibrated confidence value, and the minimum confidence
governs the safe routing tier.

Confidence above 0.95 selects a deterministic local tool or API. Confidence
from 0.60 through 0.95 selects the paired draft-and-target speculative
pipeline. Confidence below 0.60 selects the heavy autoregressive model.

Validated route targets keep deterministic tools, speculative model pairs, and
heavy models explicit. Bounded telemetry records fan-out activity and route
selection without allowing unbounded runtime growth.
