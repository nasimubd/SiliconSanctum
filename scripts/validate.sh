#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"
need jq
jq -e '.profiles | length >= 3' "$models_file" >/dev/null
for script in "$repo_dir"/scripts/*.sh "$repo_dir"/bin/local-ai; do
  zsh -o NO_BG_NICE -n < "$script"
done
python3 -m py_compile "$repo_dir/benchmarks/context_probe.py"
"$repo_dir/scripts/storage-transport.sh" | jq -e '.connection and has("thunderbolt_device_connected")' >/dev/null
print -- "Configuration, zsh syntax, and Python syntax are valid."
