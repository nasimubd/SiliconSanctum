#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

user_home="${HOME:?HOME is not set}"
bin_dir="${LOCAL_AI_BIN_DIR:-$user_home/.local/bin}"
command_path="$bin_dir/local-ai"

mkdir -p "$bin_dir"
ln -sfn "$repo_dir/bin/local-ai" "$command_path"

if [[ ":$PATH:" != *":$bin_dir:"* ]]; then
  print -u2 -- "warning: $bin_dir is not currently on PATH"
fi

"$command_path" paths
print -- "Installed $command_path"
