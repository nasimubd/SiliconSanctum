## [1.0.1](https://github.com/nasimubd/local-ai-workstation/compare/v1.0.0...v1.0.1) (2026-09-15)


### Bug Fixes

* parameterize workstation paths ([#14](https://github.com/nasimubd/local-ai-workstation/issues/14)) ([ca9b950](https://github.com/nasimubd/local-ai-workstation/commit/ca9b950315a7785bdd73385743978aae256bf637))
* preserve release commit boundary ([#21](https://github.com/nasimubd/local-ai-workstation/issues/21)) ([2690b0a](https://github.com/nasimubd/local-ai-workstation/commit/2690b0a29cb3559907edbf6b94d7babe7075f2ac))
* remove machine-specific paths ([#20](https://github.com/nasimubd/local-ai-workstation/issues/20)) ([21c96c8](https://github.com/nasimubd/local-ai-workstation/commit/21c96c88da196d6d78dcfae282f0a833c5bdd1d2))

# 1.0.0 (2026-09-15)


### Bug Fixes

* **cli:** support commands from any directory ([70ae05b](https://github.com/nasimubd/local-ai-workstation/commit/70ae05b5bfd5fec18dbb88c22fbfc310bee2d78d))
* configure Kaggle account and dataset defaults ([471c7d4](https://github.com/nasimubd/local-ai-workstation/commit/471c7d4c9443bfa7181a3a0606cf04baff256f10))
* declare semantic release repository URL ([64ffe7f](https://github.com/nasimubd/local-ai-workstation/commit/64ffe7f70399949c6cade2d4632c852dcb145f2e))
* embed Kaggle job manifest in kernel ([72ca0a0](https://github.com/nasimubd/local-ai-workstation/commit/72ca0a0425536c01d95713abf28558e36cd187c3))
* exempt pre-release legacy commit history ([4a147ab](https://github.com/nasimubd/local-ai-workstation/commit/4a147ab64df36f183cd750a29e6693ab84495d2f))
* install and verify Kaggle CLI ([eeaedb2](https://github.com/nasimubd/local-ai-workstation/commit/eeaedb2ca19464a7d841b47081c432d3834536ce))
* **server:** manage Ollama profiles idempotently ([64ab678](https://github.com/nasimubd/local-ai-workstation/commit/64ab678a9942392d6b2434c9173320343c2a2f4c))


### Features

* add quota-aware Kaggle burst worker ([a9fa251](https://github.com/nasimubd/local-ai-workstation/commit/a9fa251e48f690165115b6b455a511f96b752e98))
* expose explicit long-context Claude sessions ([c5fccc5](https://github.com/nasimubd/local-ai-workstation/commit/c5fccc5445e3cdf25a6f7ea305e4e35d84a986cf))


### Performance Improvements

* optimize local inference and benchmark backends ([2cc6152](https://github.com/nasimubd/local-ai-workstation/commit/2cc6152c8d2aaa7d3f908c235f014bc1c46825e9))

# Changelog

All notable changes to this project will be documented in this file.

Releases are generated from Conventional Commits by `mise run release:full`.
