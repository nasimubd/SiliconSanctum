#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq

profile="${1:-qwen35-4b-1m}"
print -- "Context ladder for $profile"
jq -r --arg profile "$profile" '.profiles[$profile].context_ladder[]' "$models_file" | while read -r context; do
  print -- "  $context"
done
print -- "Run each level explicitly: ./scripts/serve.sh $profile CONTEXT"
print -- "For Ollama, probe http://$AI_HOST:$AI_OLLAMA_PORT; with GGUF_PATH/llama-server, probe http://$AI_HOST:$AI_PORT."
print -- "Example: python3 benchmarks/context_probe.py --base-url http://$AI_HOST:$AI_OLLAMA_PORT --model \$(profile_value '$profile' ollama_model) --context CONTEXT --tokens TOKENS"
