# Local AI Workstation

Reproducible local inference and quantitative-development environment for an M1 Pro Mac with 16 GB unified memory and the external `TickArchive` volume.

## Profiles

- `qwen35-9b-daily`: daily quantitative coding, 128K target context.
- `qwen35-4b-1m`: experimental literal 1.01M context reader.
- `qwen38-27b-focused`: best-effort Qwen3.8 engineering profile at short context; its standard Q4 package exceeds physical memory.

## First run

```bash
cp .env.example .env
./scripts/bootstrap.sh
./scripts/doctor.sh
./scripts/storage-benchmark.sh 4
./scripts/model-pull.sh qwen35-4b-1m
./scripts/model-pull.sh qwen35-9b-daily
```

Large downloads are intentionally explicit. Do not pull Qwen3.8 until the smaller profiles and storage path are validated.

Start a profile:

```bash
./scripts/serve.sh qwen35-9b-daily
```

In another terminal, start a coding agent in the repository you want to edit:

```bash
/Users/mdnasim/epatnerlab/local-ai-workstation/scripts/agent.sh aider qwen35-9b-daily
```

The 27B profile is archived for a future machine with at least 32 GB unified memory. The launcher blocks it on this 16 GB Mac unless an explicit unsafe override is set.

For a pinned GGUF and full llama.cpp controls:

```bash
GGUF_PATH="/Volumes/TickArchive/ai-workstation/models/gguf/model.gguf" \
  ./scripts/serve.sh qwen35-4b-1m 131072
```

Increase the long-context profile only after each prior level passes retrieval, memory, swap, and latency checks. See [disaster recovery](docs/RECOVERY.md) and [Thunderbolt migration](docs/THUNDERBOLT_MIGRATION.md).

After model changes, run `make lock-models`. After committing, run `make bundle` to place a restorable Git bundle on TickArchive. For loss of both devices, push this repository to a private remote; model weights are reproducible downloads and are deliberately excluded from Git.

## Security

Servers bind to localhost by default. Never put exchange secrets in prompts, model directories, indexes, or agent configuration. Keep production trading disabled and require human authorization for every live-order action. Treat unofficial uncensored weights and model templates as untrusted code/data until independently evaluated.
