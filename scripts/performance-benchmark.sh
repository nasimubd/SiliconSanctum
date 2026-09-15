#!/bin/zsh
set -euo pipefail
zmodload zsh/datetime
source "${0:A:h}/lib.sh"

need curl
need jq
require_volume

profile="${1:-qwen35-4b-chat}"
context_override="${2:-}"
model="$(profile_value "$profile" ollama_model)"
context="${context_override:-$(jq -er --arg p "$profile" '.profiles[$p].startup_context' "$models_file")}"
endpoint="http://${AI_HOST}:${AI_OLLAMA_PORT}"

if curl -fsS --max-time 2 "$endpoint/api/version" >/dev/null 2>&1; then
  curl -fsS "$endpoint/api/ps" | jq -r '.models[]?.name' | while read -r loaded_model; do
    [[ -n "$loaded_model" ]] || continue
    jq -nc --arg model "$loaded_model" '{model:$model,keep_alive:0}' |
      curl -fsS "$endpoint/api/generate" -H 'Content-Type: application/json' -d @- >/dev/null
  done
  rm -f "$AI_ROOT/tmp/active-profile.json"
fi

started=$EPOCHREALTIME
"$repo_dir/scripts/server-manager.sh" start "$profile" "$context"
finished=$EPOCHREALTIME
activation_seconds="$(( finished - started ))"
timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
result="$AI_ROOT/benchmarks/inference-${profile}-${context}-${timestamp}.json"

python3 "$repo_dir/benchmarks/ollama_benchmark.py" \
  --base-url "$endpoint" \
  --profile "$profile" \
  --model "$model" \
  --context "$context" \
  --activation-seconds "$activation_seconds" \
  --output "$result"
print -- "Saved $result"
