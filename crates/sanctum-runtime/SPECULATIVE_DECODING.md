# Speculative decoding harness

The harness pairs a sub-2B draft model with a 7B-or-larger target model. The target verifies each draft prefix before any token is committed to session state.
