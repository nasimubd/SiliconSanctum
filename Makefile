SHELL := /bin/zsh

.PHONY: doctor storage-init storage-benchmark bootstrap validate snapshot lock-models bundle serve-daily serve-long serve-qwen38 context-ladder aider

doctor:
	./scripts/doctor.sh

storage-init:
	./scripts/storage-init.sh

storage-benchmark:
	./scripts/storage-benchmark.sh

bootstrap:
	./scripts/bootstrap.sh

validate:
	./scripts/validate.sh

snapshot:
	./scripts/snapshot.sh

lock-models:
	./scripts/lock-models.sh

bundle:
	./scripts/recovery-bundle.sh

serve-daily:
	./scripts/serve.sh qwen35-9b-daily

serve-long:
	./scripts/serve.sh qwen35-4b-1m

serve-qwen38:
	./scripts/serve.sh qwen38-27b-focused

context-ladder:
	./scripts/context-ladder.sh qwen35-4b-1m

aider:
	./scripts/agent.sh aider qwen35-9b-daily
