#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need kaggle
slug="${1:-$(jq -r .kernel_slug "$repo_dir/kaggle/job.json")}"
destination="${2:-$AI_ROOT/benchmarks/kaggle/$slug}"
mkdir -p "$destination"
kaggle kernels output "$slug" -p "$destination"
print -- "Saved Kaggle output to $destination"
