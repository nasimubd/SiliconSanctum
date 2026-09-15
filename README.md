# Local AI Workstation

Reproducible local inference and quantitative-development environment for an M1 Pro Mac with 16 GB unified memory and the external `TickArchive` volume.

## Profiles

- `qwen35-4b-chat`: fast direct chat at 8K context.
- `qwen35-4b-coding`: default coding/agent profile at 32K context.
- `qwen35-9b-daily`: higher-quality focused coding at 32K startup context.
- `qwen35-4b-1m`: experimental literal 1.01M context reader.
- `qwen38-27b-focused`: best-effort Qwen3.8 engineering profile at short context; its standard Q4 package exceeds physical memory.

## Recommended setup

Use `qwen35-4b-coding` for routine work on this 16 GB Mac; switch to `qwen35-9b-daily` only for focused quality passes. Use `qwen35-4b-1m` for staged long-context experiments. Keep `qwen38-27b-focused` archived on `TickArchive`; its weights exceed physical memory before runtime and KV-cache overhead, so the launcher blocks it unless `AI_ALLOW_UNSAFE_MODEL=1` is deliberately set.

The model server and coding agent run as separate processes. The server reads weights from `TickArchive`, loads them into unified memory, and exposes a localhost API. Aider, Claude Code integration, or a Python application connects to that API. Model storage remains external; source repositories and the reproducible workstation configuration remain in Git.

## How to use the workstation: steps 1–7

### 1. Connect and verify TickArchive

Attach the drive before starting inference, then verify the storage layout and toolchain:

```bash
ls /Volumes/TickArchive/ai-workstation
local-ai doctor
```

The external directory should contain `models`, `manifests`, `benchmarks`, `indexes`, `prompt-cache`, `research`, and `tmp`.

### 2. Start the local model server

For daily work, run:

```bash
local-ai serve daily
```

This starts or reuses a persistent Ollama LaunchAgent, unloads any previous model, and explicitly loads `qwen3.5:4b-q4_K_M` at `http://127.0.0.1:11434`. The first load is storage-bound; subsequent requests reuse the resident model and prompt cache.

For a lighter session or long-context experiment, use:

```bash
local-ai serve long
```

`serve` is idempotent: changing profiles reuses the daemon instead of trying to bind a second process to port `11434`. The 4B profile starts at a safer 128K context; request larger levels explicitly through the context ladder. Inspect or stop a managed server with:

```bash
local-ai status
local-ai stop
```

If `status` reports an external process, stop it in its original terminal with `Ctrl+C`; `local-ai` will not kill a process it does not own.

### 3. Launch a coding agent

Open terminal 2, enter the source repository the agent should work on, and start Aider:

```bash
cd /path/to/your/quant-project
/Users/mdnasim/epatnerlab/local-ai-workstation/scripts/agent.sh aider qwen35-4b-coding
```

Useful Aider commands include `/add FILE`, `/read-only FILE`, `/run pytest`, `/diff`, `/undo`, `/commit`, and `/exit`. Keep market data and credentials outside prompts and Git. Review every proposed change, especially execution, timestamp, and portfolio-accounting logic.

To try the Claude Code interface against the same local model:

```bash
cd /path/to/your/quant-project
/Users/mdnasim/epatnerlab/local-ai-workstation/scripts/agent.sh claude qwen35-4b-coding
```

For a literal long-context Claude Code session, pass the context explicitly.
Start at 524288 and advance to 1010000 only after the retrieval and memory
probes pass:

```bash
cd /path/to/your/quant-project
/Users/mdnasim/epatnerlab/local-ai-workstation/scripts/agent.sh claude qwen35-4b-1m 524288
```

Claude Code remains the local tool harness while Qwen performs inference via
Ollama's Anthropic-compatible Messages API. Qwen3.5's native limit is 262K;
524K and 1M are extrapolation experiments. Keep long sessions read-only until
retrieval accuracy and tool-call reliability are established.

### 4. Chat with the model directly

With the server running, open another terminal:

```bash
OLLAMA_HOST=http://127.0.0.1:11434 \
OLLAMA_MODELS=/Volumes/TickArchive/ai-workstation/models/ollama \
ollama run qwen3.5:9b-q4_K_M
```

Use `/bye` to leave the chat. Substitute `qwen3.5:4b-q4_K_M` when lower memory use is more important than model quality.

### 5. Connect a Python application

Ollama provides an OpenAI-compatible localhost endpoint. Add the client to a project with `uv add openai`, then use:

```python
from openai import OpenAI

client = OpenAI(base_url="http://127.0.0.1:11434/v1", api_key="ollama")
response = client.chat.completions.create(
    model="qwen3.5:9b-q4_K_M",
    messages=[
        {
            "role": "system",
            "content": (
                "Act as a quantitative-finance research reviewer. Check for "
                "leakage, overfitting, survivorship bias, invalid timestamps, "
                "and unrealistic execution assumptions."
            ),
        },
        {"role": "user", "content": "Review this backtest methodology."},
    ],
    temperature=0.1,
)
print(response.choices[0].message.content)
```

The service binds to localhost by default and has no authentication. Do not expose port `11434` to a public or untrusted network.

### 6. Validate long context progressively

Display the prescribed ladder:

```bash
local-ai context-ladder
```

Start at 128K rather than jumping to one million tokens:

```bash
local-ai serve long 131072
```

In a second terminal, run a retrieval probe:

```bash
cd /Users/mdnasim/epatnerlab/local-ai-workstation
python3 benchmarks/context_probe.py \
  --base-url http://127.0.0.1:11434 \
  --model qwen3.5:4b-q4_K_M \
  --context 131072 \
  --tokens 100000
```

Only advance after checking retrieval accuracy, memory pressure, swap use, prompt-processing time, and generation latency at `131072`, `262144`, `524288`, `786432`, and `1010000`. The model's native metadata declares 262K; larger windows are experimental extrapolation and a literal 1M KV cache may not fit comfortably in 16 GB. Retrieval and selective file loading are generally preferable for large codebases.

### 7. Preserve and recover the workstation

After changing installed models, update the committed manifests:

```bash
local-ai lock-models
```

After committing changes, refresh the offline Git bundle:

```bash
local-ai bundle
```

The bundle is stored at `/Volumes/TickArchive/ai-workstation/manifests/local-ai-workstation.bundle`. The private upstream protects the configuration if the Mac and external drive are both lost. Model weights are excluded from Git and can be downloaded again from the pinned names and manifests. See [disaster recovery](docs/RECOVERY.md) for the restoration procedure.

## Initial installation

```bash
cp .env.example .env
./scripts/bootstrap.sh
./scripts/doctor.sh
./scripts/storage-benchmark.sh 4
./scripts/model-pull.sh qwen35-4b-1m
./scripts/model-pull.sh qwen35-9b-daily
```

Large downloads are intentionally explicit. Validate the smaller profiles and storage path before archiving Qwen3.8.

Bootstrap installs `local-ai` under `~/.local/bin`, allowing workstation commands to run from any directory. Make targets remain available when the current directory is this repository; `make` does not discover this project's Makefile from the home directory.

For a pinned GGUF and full llama.cpp controls:

```bash
GGUF_PATH="/Volumes/TickArchive/ai-workstation/models/gguf/model.gguf" \
  ./scripts/serve.sh qwen35-4b-1m 131072
```

When the replacement cable arrives, follow [Thunderbolt migration](docs/THUNDERBOLT_MIGRATION.md) and benchmark the negotiated connection before changing model profiles.

## Security

Servers bind to localhost by default. Never put exchange secrets in prompts, model directories, indexes, or agent configuration. Keep production trading disabled and require human authorization for every live-order action. Treat unofficial uncensored weights and model templates as untrusted code/data until independently evaluated.
