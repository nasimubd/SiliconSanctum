#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need kaggle
job_dir="$repo_dir/kaggle"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
cp "$job_dir/kernel.py" "$job_dir/worker.py" "$job_dir/job.json" "$job_dir/kernel-metadata.json" "$tmp_dir/"
print -- "Submitting Kaggle job from $tmp_dir"
kaggle kernels push -p "$tmp_dir"
print -- "Monitor: kaggle kernels status $(jq -r .kernel_slug "$job_dir/job.json")"
