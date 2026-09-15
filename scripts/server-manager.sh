#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

need curl
need jq
need ollama
require_volume

action="${1:-status}"
endpoint="http://${AI_HOST}:${AI_OLLAMA_PORT}"
state_dir="$AI_ROOT/tmp"
pid_file="$state_dir/ollama-server.pid"
profile_file="$state_dir/active-profile.json"
log_file="$state_dir/ollama-server.log"
lock_dir="$state_dir/profile-switch.lock"
mkdir -p "$state_dir"

server_ready() {
  curl -fsS --max-time 2 "$endpoint/api/version" >/dev/null 2>&1
}

managed_pid() {
  [[ -s "$pid_file" ]] || return 1
  local pid
  pid="$(<"$pid_file")"
  [[ "$pid" == <-> ]] || return 1
  kill -0 "$pid" 2>/dev/null || return 1
  [[ "$(ps -p "$pid" -o command= 2>/dev/null)" == *"ollama serve"* ]] || return 1
  print -- "$pid"
}

start_server() {
  if server_ready; then
    print -- "Reusing Ollama already listening at $endpoint."
    return
  fi

  rm -f "$pid_file"
  print -- "Starting managed Ollama server at $endpoint."
  OLLAMA_HOST="$endpoint" \
  OLLAMA_MODELS="$AI_ROOT/models/ollama" \
  OLLAMA_FLASH_ATTENTION=1 \
  OLLAMA_KV_CACHE_TYPE=q4_0 \
  OLLAMA_NUM_PARALLEL=1 \
  nohup ollama serve > "$log_file" 2>&1 < /dev/null &!
  server_pid=$!
  print -- "$server_pid" > "$pid_file"

  for _ in {1..60}; do
    server_ready && return
    sleep 1
  done
  tail -40 "$log_file" >&2 || true
  die "Ollama did not become ready; inspect $log_file"
}

load_profile() {
  local profile="${1:-qwen35-9b-daily}"
  local context_override="${2:-}"
  local model context payload response minimum_ram_gib physical_ram_bytes physical_ram_gib

  model="$(profile_value "$profile" ollama_model)"
  context="${context_override:-$(jq -er --arg p "$profile" '.profiles[$p].startup_context // .profiles[$p].context' "$models_file")}"
  [[ "$context" == <-> && "$context" -gt 0 ]] || die "context must be a positive integer"

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

  mkdir "$lock_dir" 2>/dev/null || die "another profile switch is already in progress"
  trap 'rmdir "$lock_dir" 2>/dev/null || true' EXIT

  start_server
  if ! curl -fsS "$endpoint/api/tags" | jq -e --arg model "$model" '.models[]?.name == $model' >/dev/null; then
    die "$model is unavailable to the running server. Stop the external Ollama process, then retry so the managed server can use $AI_ROOT/models/ollama."
  fi

  if [[ -s "$profile_file" ]] &&
     jq -e --arg profile "$profile" --arg model "$model" --argjson context "$context" \
       '.profile == $profile and .model == $model and .context == $context' "$profile_file" >/dev/null &&
     curl -fsS "$endpoint/api/ps" | jq -e --arg model "$model" '.models[]?.name == $model' >/dev/null; then
    print -- "Already ready: $model at $endpoint (context $context)."
    return
  fi

  print -- "Unloading resident models before allocating the requested context."
  rm -f "$profile_file"
  curl -fsS "$endpoint/api/ps" | jq -r '.models[]?.name' | while read -r loaded_model; do
    [[ -n "$loaded_model" ]] || continue
    jq -nc --arg model "$loaded_model" '{model:$model,keep_alive:0}' |
      curl -fsS "$endpoint/api/generate" -H 'Content-Type: application/json' -d @- >/dev/null
  done

  print -- "Loading $profile ($model) with context $context. This can take several minutes from external storage."
  payload="$(jq -nc --arg model "$model" --argjson context "$context" \
    '{model:$model,prompt:"",stream:false,keep_alive:-1,options:{num_ctx:$context,num_predict:1}}')"
  response="$(curl -fsS --max-time 1800 "$endpoint/api/generate" -H 'Content-Type: application/json' -d "$payload")"
  error="$(jq -r '.error // empty' <<< "$response")"
  [[ -z "$error" ]] || die "Ollama could not load the profile: $error"

  jq -n --arg profile "$profile" --arg model "$model" --argjson context "$context" \
    --arg activated_at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
    '{profile:$profile,model:$model,context:$context,activated_at:$activated_at}' > "$profile_file"
  print -- "Ready: $model at $endpoint (context $context, keep-alive unlimited)."
}

show_status() {
  if ! server_ready; then
    print -- "Ollama server: stopped"
    return 1
  fi
  print -- "Ollama server: running at $endpoint"
  if pid="$(managed_pid 2>/dev/null)"; then
    print -- "Ownership: managed by local-ai (PID $pid)"
  else
    print -- "Ownership: external process"
  fi
  [[ -s "$profile_file" ]] && jq . "$profile_file"
  curl -fsS "$endpoint/api/ps" | jq '{loaded_models:[.models[]? | {name,size,parameter_size,quantization_level,expires_at,size_vram}]}'
}

stop_server() {
  local pid
  if pid="$(managed_pid 2>/dev/null)"; then
    kill "$pid"
    for _ in {1..50}; do
      kill -0 "$pid" 2>/dev/null || break
      sleep 0.1
    done
    rm -f "$pid_file" "$profile_file"
    print -- "Stopped managed Ollama server (PID $pid)."
  elif server_ready; then
    die "Ollama is running but was not started by local-ai. Stop it with Ctrl+C in its original terminal, or quit its owning application."
  else
    rm -f "$pid_file" "$profile_file"
    print -- "Ollama server is already stopped."
  fi
}

case "$action" in
  start) load_profile "${2:-qwen35-9b-daily}" "${3:-}" ;;
  status) show_status ;;
  stop) stop_server ;;
  *) die "unknown server action '$action' (supported: start, status, stop)" ;;
esac
