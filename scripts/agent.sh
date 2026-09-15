#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq

agent="${1:-aider}"
profile="${2:-qwen35-9b-daily}"
model="$(profile_value "$profile" ollama_model)"

case "$agent" in
  aider)
    need aider
    export OLLAMA_API_BASE="http://${AI_HOST}:${AI_OLLAMA_PORT}"
    exec aider --model "ollama_chat/$model" --config "$repo_dir/config/aider.conf.yml"
    ;;
  claude)
    need ollama
    exec ollama launch claude --model "$model"
    ;;
  *)
    die "unknown agent '$agent' (supported: aider, claude)"
    ;;
esac
