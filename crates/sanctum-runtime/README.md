# Sanctum Runtime

The runtime crate owns domain-neutral model process supervision.

Process specifications validate executable, model, environment, arguments, and
working directory inputs before launch. Supervised children track backend
lifecycle state and perform bounded shutdown: request termination, await the
configured grace period, then force termination only after the deadline expires.

Backends persist their KV cache before exiting and publish a cache marker at an
absolute path. The marker API lets higher-level orchestration verify persistence
without coupling the runtime to a model implementation.
