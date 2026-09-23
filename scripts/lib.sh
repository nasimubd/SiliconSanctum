#!/bin/zsh
set -euo pipefail

repo_dir="${0:A:h:h}"
ai_root_from_environment="${AI_ROOT-}"
if [[ -f "$repo_dir/.env" ]]; then
  set -a
  source "$repo_dir/.env"
  set +a
fi

if [[ -z "$ai_root_from_environment" && -f "$repo_dir/.state/ai-root" ]]; then
  IFS= read -r AI_ROOT < "$repo_dir/.state/ai-root"
fi

: "${AI_VOLUME:=/Volumes/AI-NVME}"
: "${AI_ROOT:=$AI_VOLUME/silicon-sanctum}"
: "${AI_HOST:=127.0.0.1}"
: "${AI_PORT:=8080}"
: "${AI_OLLAMA_PORT:=11434}"
: "${AI_MLX_PORT:=8081}"
: "${AI_DEFAULT_CONTEXT:=32768}"
: "${AI_THREADS:=6}"
: "${AI_PARALLEL:=1}"

models_file="$repo_dir/config/models.json"

die() {
  print -u2 -- "error: $*"
  exit 1
}

need() {
  command -v "$1" >/dev/null 2>&1 || die "required command not found: $1"
}

profile_value() {
  local profile="$1" key="$2"
  jq -er --arg profile "$profile" --arg key "$key" '.profiles[$profile][$key]' "$models_file"
}

require_volume() {
  [[ -d "$AI_VOLUME" ]] || die "AI volume is not mounted at $AI_VOLUME"
}
