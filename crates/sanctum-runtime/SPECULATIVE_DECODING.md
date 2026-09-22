# Speculative decoding harness

The harness pairs a sub-2B draft model with a 7B-or-larger target model. The target verifies each draft prefix before any token is committed to session state.

## Verify width

The scheduler narrows verification batches as context pressure rises. It selects the minimum width immediately when measured unified-memory bandwidth reaches 90 percent utilization.
