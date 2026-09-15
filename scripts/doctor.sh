#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

failures=0
for tool in git jq uv cmake ollama llama-server kaggle; do
  if command -v "$tool" >/dev/null 2>&1; then
    print -- "ok      $tool: $(command -v "$tool")"
  else
    print -- "missing $tool"
    (( failures += 1 ))
  fi
done

if [[ -d "$AI_VOLUME" ]]; then
  print -- "ok      volume: $AI_VOLUME"
  df -h "$AI_VOLUME" | tail -1
else
  print -- "missing volume: $AI_VOLUME"
  (( failures += 1 ))
fi

print -- "hardware"
system_profiler SPHardwareDataType 2>/dev/null | rg 'Model Name|Chip:|Total Number of Cores|Memory:' || true

if (( failures > 0 )); then
  die "$failures required checks failed; run ./scripts/bootstrap.sh"
fi
print -- "Doctor passed."
