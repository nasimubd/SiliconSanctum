#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need git

git -C "$repo_dir" rev-parse --verify HEAD >/dev/null 2>&1 || die "commit the repository before creating a recovery bundle"
destination="$AI_ROOT/manifests/local-ai-workstation.bundle"
git -C "$repo_dir" bundle create "$destination" --all
git -C "$repo_dir" bundle verify "$destination"
print -- "Recovery bundle written to $destination"
