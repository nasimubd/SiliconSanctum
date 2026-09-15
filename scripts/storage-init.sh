#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
require_volume

mkdir -p \
  "$AI_ROOT/models/ollama" \
  "$AI_ROOT/models/gguf" \
  "$AI_ROOT/models/mlx" \
  "$AI_ROOT/indexes" \
  "$AI_ROOT/prompt-cache" \
  "$AI_ROOT/benchmarks" \
  "$AI_ROOT/manifests" \
  "$AI_ROOT/research" \
  "$AI_ROOT/tmp"

print -- "Initialized $AI_ROOT"
