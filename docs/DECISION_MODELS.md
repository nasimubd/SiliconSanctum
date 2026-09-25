# Decision models and Jev

## What a decision plane does

Generative models are expensive and probabilistic. A small decision model can
handle narrow, typed questions before or after generation:

- route a request to a fast, strong, or remote model;
- classify intent and tool requirements;
- score risk, ambiguity, or evidence quality;
- choose whether a tool call needs approval;
- verify structured output and decide whether to retry or escalate.

Rust remains the authority for thresholds, permissions, side effects, and
fallbacks. A decision model proposes a typed result with confidence; it does
not execute trades or tools by itself.

## Verified Jev status

TypeSafe's official materials describe **Jev** as its first System One model,
accessed through the TypeSafe API. The documented product is hosted and the
model weights are not published as an open-weight local checkpoint. The
official SDKs and the
[`system-one-adapter-python`](https://github.com/typesafe-ai/system-one-adapter-python)
repository are open source, but the adapter is a provider-backed compatibility
and development layer; it is not a FOSS implementation of Jev's weights.

Therefore Silicon Sanctum must not claim that Jev can run locally or that it is
an open-source decision model. It can be an optional remote decision provider,
subject to credentials, network availability, privacy policy, version pinning,
and calibration checks.

Sources checked 2026-09-25:

- [TypeSafe introduction](https://docs.typesafe.ai/introduction)
- [TypeSafe models](https://docs.typesafe.ai/models)
- [TypeSafe API](https://docs.typesafe.ai/api)
- [System One concepts](https://docs.typesafe.ai/concepts/system-one)
- [TypeSafe launch announcement](https://typesafe.ai/blog/introducing-system-one-models-and-jev)

## Local FOSS implementation status

There is no verified FOSS implementation of Jev's model weights. The current
repository's decision module is a backend-neutral contract with validation,
latency budgets, logits-to-probability helpers, and a fixture executor. It is
not yet connected to a real local model.

The FOSS implementation we should utilize is therefore a local decision-plane
backend, not Jev itself. The practical candidates are:

| Candidate | Current status | Role |
|---|---|---|
| Core ML | Available on Apple platforms, FOSS integration surface varies by model/tooling | Fast Apple-native small classifier/extractor |
| ONNX Runtime | Mature open-source runtime with broad model export support | Portable baseline for typed decisions |
| Candle | Open-source Rust ML framework | Pure-Rust option where the model architecture is supported |
| TypeSafe system-one-adapter | Open source, provider-backed | Jev-shaped development/comparison adapter, not local Jev |

The selection gate is latency, memory, calibration, and disagreement quality on
our own labelled routing/tool-safety data. Until that benchmark exists, no
candidate should be declared the production decision model.

## Local FOSS path

The existing `sanctum-runtime` decision contract is the correct abstraction,
but its current executor is backend-neutral rather than a shipped Jev clone.
The next local implementation should benchmark small ONNX, Core ML, or Candle
classifiers/encoders against typed `Choice`, `Score`, and boolean probability
outputs. Candidate models must be evaluated on Silicon Sanctum's own labelled
routing, tool-safety, and verification data; no generic benchmark proves they
are safe for these decisions.

Jev can serve as a teacher or comparison provider. If it is used to label data,
record the Jev version, prompt/schema, threshold policy, and disagreement cases.
Do not distill or redistribute TypeSafe outputs without checking its current
terms.

## How this improves speed and robustness

The decision plane can make a small local model feel more capable by sending
easy work to a fast path, using speculative verification, rejecting malformed
tool calls before execution, and escalating uncertain cases to a stronger
model. It cannot make an open-weight model identical to a frontier model.
The honest product target is frontier-like workflow behavior: low latency on
routine requests, predictable tool use, typed fallbacks, and measured quality.
