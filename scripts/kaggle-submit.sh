#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need kaggle
job_dir="$repo_dir/kaggle"
tmp_dir="$(mktemp -d)"
trap 'rm -rf "$tmp_dir"' EXIT
cp "$job_dir/kernel.py" "$job_dir/worker.py" "$job_dir/job.json" "$job_dir/kernel-metadata.json" "$tmp_dir/"
owner="$(kaggle config view | awk '/username:/ {print $3; exit}')"
[[ -n "$owner" ]] || die "could not determine Kaggle username; run kaggle auth login"
slug="$owner/$(jq -r .kernel_slug "$job_dir/job.json")"
jq --arg slug "$slug" '.kernel_slug=$slug' "$tmp_dir/job.json" > "$tmp_dir/job.json.tmp"
mv "$tmp_dir/job.json.tmp" "$tmp_dir/job.json"
jq --arg id "$slug" '.id=$id' "$tmp_dir/kernel-metadata.json" > "$tmp_dir/kernel-metadata.json.tmp"
mv "$tmp_dir/kernel-metadata.json.tmp" "$tmp_dir/kernel-metadata.json"
print -- "Submitting Kaggle job from $tmp_dir"
kaggle kernels push -p "$tmp_dir"
print -- "Monitor: kaggle kernels status $slug"
