#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq

profile="${1:-qwen35-9b-daily}"
context_override="${2:-}"
context="${context_override:-$(profile_value "$profile" context)}"
cache_k="$(profile_value "$profile" cache_type_k)"
cache_v="$(profile_value "$profile" cache_type_v)"
minimum_ram_gib="$(jq -r --arg p "$profile" '.profiles[$p].minimum_ram_gib // 0' "$models_file")"
physical_ram_bytes="$(sysctl -n hw.memsize 2>/dev/null || true)"
if [[ "$physical_ram_bytes" == <-> ]]; then
  physical_ram_gib="$(( physical_ram_bytes / 1024 / 1024 / 1024 ))"
else
  physical_ram_gib="$(hostinfo | awk '/Primary memory available:/ {printf "%d", $4}')"
fi
[[ "$physical_ram_gib" == <-> ]] || die "could not determine physical memory safely"

if (( physical_ram_gib < minimum_ram_gib )) && [[ "${AI_ALLOW_UNSAFE_MODEL:-0}" != "1" ]]; then
  die "$profile requires at least ${minimum_ram_gib} GiB by policy; this Mac has ${physical_ram_gib} GiB. Set AI_ALLOW_UNSAFE_MODEL=1 only for an intentional experiment."
fi

gguf_path="${GGUF_PATH:-}"
if [[ -z "$gguf_path" ]]; then
  model="$(profile_value "$profile" ollama_model)"
  need ollama
  print -- "Starting Ollama profile $profile ($model), requested context $context"
  print -- "For exact llama.cpp cache controls, set GGUF_PATH to a pinned GGUF file."
  export OLLAMA_MODELS="$AI_ROOT/models/ollama"
  export OLLAMA_CONTEXT_LENGTH="$context"
  export OLLAMA_FLASH_ATTENTION=1
  export OLLAMA_KV_CACHE_TYPE="$cache_k"
  export OLLAMA_NUM_PARALLEL=1
  exec ollama serve
fi

need llama-server
[[ -f "$gguf_path" ]] || die "GGUF_PATH does not exist: $gguf_path"
print -- "Starting llama.cpp profile $profile with context $context"
exec llama-server \
  --model "$gguf_path" \
  --host "$AI_HOST" \
  --port "$AI_PORT" \
  --ctx-size "$context" \
  --parallel 1 \
  --threads "$AI_THREADS" \
  --n-gpu-layers auto \
  --flash-attn on \
  --cache-type-k "$cache_k" \
  --cache-type-v "$cache_v" \
  --load-mode mmap \
  --no-mmproj
