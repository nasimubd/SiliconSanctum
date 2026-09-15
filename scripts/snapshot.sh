#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
mkdir -p "$repo_dir/.state"

{
  print -- "captured_at=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
  print -- "macos=$(sw_vers -productVersion)"
  print -- "architecture=$(uname -m)"
  for tool in brew cmake uv python3 node ollama llama-server; do
    if command -v "$tool" >/dev/null 2>&1; then
      print -- "$tool=$($tool --version 2>&1 | head -1)"
    fi
  done
} > "$repo_dir/.state/versions.txt"

brew bundle dump --file "$repo_dir/.state/Brewfile.lock" --force
if command -v ollama >/dev/null 2>&1; then
  OLLAMA_MODELS="$AI_ROOT/models/ollama" ollama list > "$repo_dir/.state/ollama-models.txt" 2>/dev/null || true
fi
print -- "Snapshot written to $repo_dir/.state (intentionally git-ignored; copy chosen manifests into config/locks to pin them)."
