#!/bin/zsh
set -euo pipefail

repo_dir="${0:A:h:h}"
if [[ -f "$repo_dir/.env" ]]; then
  set -a
  source "$repo_dir/.env"
  set +a
fi

: "${AI_VOLUME:=/Volumes/TickArchive}"
: "${AI_ROOT:=$AI_VOLUME/ai-workstation}"
: "${AI_HOST:=127.0.0.1}"
: "${AI_PORT:=8080}"
: "${AI_OLLAMA_PORT:=11434}"
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
