#!/bin/zsh
set -euo pipefail
source "${0:A:h}/lib.sh"

need jq
need git
need uv
require_volume

registry="$repo_dir/config/backtests.json"
action="${1:-list}"
project="${2:-}"
job="${3:-}"
allow_dirty=0
dry_run=0

for arg in "$@"; do
  case "$arg" in
    --allow-dirty) allow_dirty=1 ;;
    --dry-run) dry_run=1 ;;
  esac
done

list_jobs() {
  jq -r '.projects | to_entries[] | .key as $project | .value.jobs | to_entries[] | "\($project) \(.key)\t\(.value.description)"' "$registry"
}

[[ "$action" == "list" ]] && { list_jobs; exit 0; }
[[ "$action" == "run" && -n "$project" && -n "$job" ]] || die "usage: local-ai backtest list | local-ai backtest run PROJECT JOB [--allow-dirty] [--dry-run]"

jq -e --arg project "$project" --arg job "$job" '.projects[$project].jobs[$job]' "$registry" >/dev/null || die "unknown project/job; run local-ai backtest list"
directory_env="$(jq -er --arg project "$project" '.projects[$project].directory_env' "$registry")"
fallback_directory="$(jq -er --arg project "$project" '.projects[$project].fallback_directory' "$registry")"
project_dir="${(P)directory_env:-$repo_dir/$fallback_directory}"
if [[ -d "$PWD/.git" && "$PWD" == */msys-alpha-forage ]]; then
  project_dir="$PWD"
fi
[[ -d "$project_dir/.git" ]] || die "$project directory is unavailable: $project_dir (set $directory_env to its checkout)"

data_roots=("${(@f)$(jq -r --arg project "$project" '.projects[$project].data_roots[]' "$registry")}")
for data_root in "${data_roots[@]}"; do
  [[ -d "$data_root" ]] || die "required TickArchive data root is unavailable: $data_root"
done

if (( ! allow_dirty )) && [[ -n "$(git -C "$project_dir" status --porcelain)" ]]; then
  die "$project has uncommitted changes; commit/stash them or rerun with --allow-dirty"
fi

endpoint="http://${AI_HOST}:${AI_OLLAMA_PORT}"
if curl -fsS --max-time 2 "$endpoint/api/ps" 2>/dev/null | jq -e '(.models // []) | length > 0' >/dev/null; then
  die "an inference model is resident; stop it with local-ai stop before a backtest so the 16 GiB memory budget is not contested"
fi

command=("${(@f)$(jq -r --arg project "$project" --arg job "$job" '.projects[$project].jobs[$job].command[]' "$registry")}")
timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
run_dir="$AI_ROOT/benchmarks/backtests/${project}-${job}-${timestamp}"
manifest="$run_dir/manifest.json"

git_sha="$(git -C "$project_dir" rev-parse HEAD)"
git_dirty="$(git -C "$project_dir" status --porcelain | wc -l | tr -d ' ')"
command_json="$(printf '%s\n' "${command[@]}" | jq -R . | jq -s .)"
data_json="$(jq -c --arg project "$project" '.projects[$project].data_roots' "$registry")"

write_manifest() {
  local exit_code="$1" finished_at="$2"
  jq -n \
    --arg schema_version "1" \
    --arg project "$project" \
    --arg job "$job" \
    --arg project_dir "$project_dir" \
    --arg git_sha "$git_sha" \
    --arg started_at "$started_at" \
    --arg finished_at "$finished_at" \
    --argjson exit_status "$exit_code" \
    --argjson git_dirty_files "$git_dirty" \
    --argjson command "$command_json" \
    --argjson data_roots "$data_json" \
    '{schema_version:($schema_version|tonumber),project:$project,job:$job,project_dir:$project_dir,git_sha:$git_sha,started_at:$started_at,finished_at:$finished_at,exit_status:$exit_status,git_dirty_files:$git_dirty_files,command:$command,data_roots:$data_roots}' > "$manifest"
}

print -- "Project: $project_dir @ $git_sha"
print -- "Job: $job"
print -- "Command: ${(j: :)command}"
print -- "Artifacts: $run_dir"
if (( dry_run )); then
  print -- "Dry run; no backtest was executed."
  exit 0
fi

mkdir -p "$run_dir"

started_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
set +e
(
  cd "$project_dir"
  "${command[@]}"
) 2>&1 | tee "$run_dir/output.log"
exit_code=${pipestatus[1]}
set -e
finished_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
write_manifest "$exit_code" "$finished_at"
print -- "Saved $manifest"
exit "$exit_code"
