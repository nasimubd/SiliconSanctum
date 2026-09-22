SHELL := /bin/zsh

.PHONY: doctor storage-init storage-benchmark migrate-thunderbolt backtest bootstrap install-cli validate snapshot lock-models bundle serve-daily serve-long serve-qwen38 context-ladder aider kaggle-submit kaggle-output core-check core-test core-lint

core-check:
	cargo check --workspace

core-test:
	cargo test --workspace

core-lint:
	cargo fmt --all -- --check
	cargo clippy --workspace --all-targets -- -D warnings

doctor:
	./scripts/doctor.sh

storage-init:
	./scripts/storage-init.sh

storage-benchmark:
	./scripts/storage-benchmark.sh

migrate-thunderbolt:
	./scripts/thunderbolt-migration.sh

backtest:
	./scripts/backtest.sh list

bootstrap:
	./scripts/bootstrap.sh

install-cli:
	./scripts/install-cli.sh

validate:
	./scripts/validate.sh

snapshot:
	./scripts/snapshot.sh

lock-models:
	./scripts/lock-models.sh

bundle:
	./scripts/recovery-bundle.sh

serve-daily:
	./scripts/server-manager.sh start qwen35-4b-coding

serve-long:
	./scripts/server-manager.sh start qwen35-4b-1m

serve-qwen38:
	./scripts/server-manager.sh start qwen38-27b-focused

context-ladder:
	./scripts/context-ladder.sh qwen35-4b-1m

aider:
	./scripts/agent.sh aider qwen35-9b-daily

kaggle-submit:
	./scripts/kaggle-submit.sh

kaggle-output:
	./scripts/kaggle-output.sh
