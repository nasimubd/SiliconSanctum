#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

need brew
brew bundle --file "$repo_dir/Brewfile"

if [[ ! -f "$repo_dir/.env" ]]; then
  cp "$repo_dir/.env.example" "$repo_dir/.env"
fi

"$repo_dir/scripts/storage-init.sh"
"$repo_dir/scripts/install-cli.sh"

if command -v uv >/dev/null 2>&1; then
  # Aider's current voice dependency still imports audioop, removed in Python 3.13.
  uv tool install --upgrade --python 3.12 aider-chat
  uv tool install --upgrade huggingface-hub
fi

print -- "Bootstrap complete. Run from any directory: local-ai doctor"
