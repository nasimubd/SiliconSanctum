#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need ollama
need jq

profile="${1:-qwen35-9b-daily}"
model="$(profile_value "$profile" ollama_model)"
if curl -fsS http://127.0.0.1:11434/api/version >/dev/null 2>&1; then
  die "an Ollama server is already running; stop it, then rerun this command so OLLAMA_MODELS points to the external volume"
fi
print -- "Pulling $model for profile $profile"
OLLAMA_MODELS="$AI_ROOT/models/ollama" ollama serve > "$AI_ROOT/tmp/ollama-pull.log" 2>&1 &
server_pid=$!
trap 'kill "$server_pid" 2>/dev/null || true' EXIT
for _ in {1..60}; do
  curl -fsS http://127.0.0.1:11434/api/version >/dev/null 2>&1 && break
  sleep 1
done
curl -fsS http://127.0.0.1:11434/api/version >/dev/null 2>&1 || die "temporary Ollama server did not start"
ollama pull "$model"
ollama show "$model" --modelfile > "$AI_ROOT/manifests/${profile}.Modelfile"
ollama show "$model" --json > "$AI_ROOT/manifests/${profile}.json" 2>/dev/null || true
print -- "Captured model manifest under $AI_ROOT/manifests"
