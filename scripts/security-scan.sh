#!/bin/zsh
set -euo pipefail

# Scan tracked files and all local refs for common credential material.
# This is a guardrail, not a substitute for rotating credentials.
repo_dir="${0:A:h:h}"
cd "$repo_dir"

patterns='(BEGIN [A-Z ]*PRIVATE KEY|gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{20,}|KAGGLE_API_TOKEN|KAGGLE_KEY|AWS_SECRET_ACCESS_KEY|api[_-]?key[[:space:]]*[:=][[:space:]]*["'\''"]?[A-Za-z0-9_-]{12,}|password[[:space:]]*[:=][[:space:]]*["'\''"]?[^[:space:]\"'\''"]{8,})'

matches="$(git grep -n -I -E "$patterns" HEAD -- 2>/dev/null || true)"
while IFS= read -r ref; do
  [[ -n "$ref" ]] || continue
  matches+="$(git grep -n -I -E "$patterns" "$ref" -- 2>/dev/null || true)"
done < <(git for-each-ref --format='%(refname)')

if [[ -n "$matches" ]]; then
  print -u2 -- "Potential credential material found:"
  print -u2 -- "$matches"
  exit 1
fi

print -- "No common credential patterns found in tracked files or local refs."
