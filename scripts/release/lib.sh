#!/usr/bin/env bash
set -euo pipefail

check_clean_tree() {
  if [ -n "$(git status --porcelain)" ]; then
    echo "working directory is not clean" >&2
    git status --short >&2
    return 1
  fi
  echo "working directory clean"
}

check_on_main() {
  [ "$(git branch --show-current)" = "main" ] || {
    echo "release must run on main" >&2
    return 1
  }
  echo "on main"
}

ensure_gh_token() {
  if [ -z "${GH_TOKEN:-}" ]; then
    command -v gh >/dev/null 2>&1 || { echo "install gh or set GH_TOKEN" >&2; return 1; }
    GH_TOKEN="$(gh auth token)"
    export GH_TOKEN
  fi
  export GITHUB_TOKEN="$GH_TOKEN"
  echo "GitHub token available"
}

check_commits_conventional() {
  local bad=0
  while IFS= read -r hash; do
    subject="$(git log -1 --format=%s "$hash")"
    if ! printf '%s\n' "$(git log -1 --format=%B "$hash")" | node scripts/release/commitlint.cjs; then
      echo "non-conventional commit: $hash $subject" >&2
      bad=$((bad + 1))
    fi
  done < <(git log --format='%H' "${1:-HEAD}")
  [ "$bad" -eq 0 ] || return 1
  echo "all commits are conventional"
}
