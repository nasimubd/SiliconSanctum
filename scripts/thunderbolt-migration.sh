#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq
require_volume

size_gib="${1:-16}"
[[ "$size_gib" =~ '^[0-9]+$' ]] || die "size must be an integer GiB"

print -- "Transport detected:"
"$repo_dir/scripts/storage-transport.sh" | jq .
print -- ""
print -- "Running two ${size_gib} GiB storage passes. Each pass creates and removes one disposable file."
for pass in 1 2; do
  print -- "Migration benchmark pass $pass/2"
  "$repo_dir/scripts/storage-benchmark.sh" "$size_gib"
done
print -- "Review both saved JSON files, then validate sleep/wake and a multi-hour workload before changing model profiles."
