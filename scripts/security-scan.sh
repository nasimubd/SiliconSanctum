#!/usr/bin/env bash
set -euo pipefail

# Scan tracked files and all local refs for common credential material.
# This is a guardrail, not a substitute for rotating credentials.
repo_dir="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_dir"

patterns='(BEGIN [A-Z ]*PRIVATE KEY|gh[pousr]_[A-Za-z0-9_]{20,}|github_pat_[A-Za-z0-9_]{20,}|AKIA[0-9A-Z]{16}|AIza[0-9A-Za-z_-]{20,}|KAGGLE_API_TOKEN|KAGGLE_KEY|AWS_SECRET_ACCESS_KEY|api[_-]?key[[:space:]]*[:=][[:space:]]*["'\''"]?[A-Za-z0-9_-]{12,}|password[[:space:]]*[:=][[:space:]]*["'\''"]?[^[:space:]\"'\''"]{8,})'

matches="$(git grep -n -I -E "$patterns" HEAD -- ':!scripts/security-scan.sh' 2>/dev/null || true)"
while IFS= read -r ref; do
  [[ -n "$ref" ]] || continue
  matches+="$(git grep -n -I -E "$patterns" "$ref" -- ':!scripts/security-scan.sh' 2>/dev/null || true)"
done < <(git for-each-ref --format='%(refname)')

if [[ -n "$matches" ]]; then
  printf '%s\n' "Potential credential material found:" >&2
  printf '%s\n' "$matches" >&2
  exit 1
fi

printf '%s\n' "No common credential patterns found in tracked files or local refs."
