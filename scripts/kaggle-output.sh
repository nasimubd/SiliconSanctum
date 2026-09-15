#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need kaggle
owner="$(kaggle config view | awk '/username:/ {print $3; exit}')"
if [[ -n "${1:-}" ]]; then
  slug="$1"
else
  slug="$owner/$(jq -r .kernel_slug "$repo_dir/kaggle/job.json")"
fi
destination="${2:-$AI_ROOT/benchmarks/kaggle/$slug}"
mkdir -p "$destination"
kaggle kernels output "$slug" -p "$destination"
print -- "Saved Kaggle output to $destination"
