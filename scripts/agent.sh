#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq

agent="${1:-aider}"
profile="${2:-qwen35-4b-coding}"
context_override="${3:-}"
model="$(profile_value "$profile" ollama_model)"

"$repo_dir/scripts/server-manager.sh" start "$profile" "$context_override"

case "$agent" in
  aider)
    need aider
    export OLLAMA_API_BASE="http://${AI_HOST}:${AI_OLLAMA_PORT}"
    exec aider --model "ollama_chat/$model" --editor-model "ollama_chat/$model" \
      --weak-model "ollama_chat/$model" --config "$repo_dir/config/aider.conf.yml"
    ;;
  claude)
    need claude
    export ANTHROPIC_AUTH_TOKEN=ollama
    export ANTHROPIC_BASE_URL="http://${AI_HOST}:${AI_OLLAMA_PORT}"
    export ANTHROPIC_MODEL="$model"
    export ANTHROPIC_SMALL_FAST_MODEL="$model"
    exec claude --model "$model"
    ;;
  *)
    die "unknown agent '$agent' (supported: aider, claude)"
    ;;
esac
