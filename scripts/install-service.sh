#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

need launchctl
need plutil
need ollama
require_volume

service_label="com.nasim.siliconsanctum.ollama"
service_domain="gui/$(id -u)"
service_target="$service_domain/$service_label"
user_home="${HOME:?HOME is not set}"
launch_agents="$user_home/Library/LaunchAgents"
plist="$launch_agents/$service_label.plist"
template="$repo_dir/config/$service_label.plist"
action="${1:-start}"

install_plist() {
  mkdir -p "$launch_agents"
  mkdir -p "$user_home/Library/Logs/silicon-sanctum"
  cp "$template" "$plist"
  plutil -replace ProgramArguments -json '["/bin/zsh","-lc","exec /opt/homebrew/bin/ollama serve"]' "$plist"
  plutil -replace EnvironmentVariables.HOME -string "$user_home" "$plist"
  plutil -replace EnvironmentVariables.OLLAMA_CONTEXT_LENGTH -string "$AI_DEFAULT_CONTEXT" "$plist"
  plutil -replace EnvironmentVariables.OLLAMA_HOST -string "http://${AI_HOST}:${AI_OLLAMA_PORT}" "$plist"
  plutil -replace EnvironmentVariables.OLLAMA_MODELS -string "$AI_ROOT/models/ollama" "$plist"
  plutil -replace StandardErrorPath -string "$user_home/Library/Logs/silicon-sanctum/ollama-server.error.log" "$plist"
  plutil -replace StandardOutPath -string "$user_home/Library/Logs/silicon-sanctum/ollama-server.log" "$plist"
  plutil -lint "$plist" >/dev/null
}

case "$action" in
  start)
    install_plist
    if launchctl print "$service_target" >/dev/null 2>&1; then
      launchctl bootout "$service_target" >/dev/null 2>&1 || true
    fi
    launchctl bootstrap "$service_domain" "$plist"
    launchctl kickstart "$service_target"
    ;;
  stop)
    if launchctl print "$service_target" >/dev/null 2>&1; then
      launchctl bootout "$service_target"
    fi
    ;;
  status)
    launchctl print "$service_target"
    ;;
  *) die "unknown service action '$action' (supported: start, stop, status)" ;;
esac
