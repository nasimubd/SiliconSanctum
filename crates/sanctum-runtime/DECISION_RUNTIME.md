# System One Decision Runtime

The decision runtime accepts normalized finite inputs and executes a bounded
sub-200M-parameter model through a backend-neutral executor contract. Runtime
measurements classify executions against the required 70–500 ms latency window.
ONNX, CoreML, and Candle are represented as supported backend targets.

`Noul` is a calibrated boolean probability constrained to `[0.0, 1.0]`.
Stable sigmoid conversion handles positive and negative logits, while bounded
logarithmic scoring avoids undefined values at exact probability boundaries.

`Choice` normalizes finite logits into a categorical distribution of no more
than 255 unique schemas. Stable softmax prevents overflow and exposes both the
winning schema and per-schema probabilities.

`Score` interpolates across a strictly ordered ordinal rubric. Inputs outside
the rubric clamp to its endpoints, providing deterministic calibrated values.

Later routing parts consume these primitives concurrently for intent, tooling,
and complexity decisions.
